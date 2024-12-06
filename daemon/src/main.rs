// TODO: Implement daemon for managing the robot.
// This will run the gRPC server and, if applicable, a runtime loop
// (e.g., actuator polling, loaded model inference).

use chrono::Local;
use clap::Parser;
use directories::BaseDirs;
use eyre::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use kos_core::google_proto::longrunning::operations_server::OperationsServer;
use kos_core::services::OperationsServiceImpl;
use kos_core::telemetry::Telemetry;
use kos_core::Platform;
use kos_core::ServiceEnum;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

#[cfg(not(any(feature = "kos-sim", feature = "kos-zeroth-01", feature = "kos-kbot")))]
use kos_stub::StubPlatform as PlatformImpl;

#[cfg(feature = "kos-sim")]
use kos_sim::SimPlatform as PlatformImpl;

#[cfg(feature = "kos-zeroth-01")]
use kos_zeroth_01::Zeroth01Platform as PlatformImpl;

#[cfg(feature = "kos-kbot")]
use kos_kbot::KbotPlatform as PlatformImpl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable file logging
    #[arg(long, default_value_t = false)]
    log: bool,
}

fn add_service_to_router(
    router: tonic::transport::server::Router,
    service: ServiceEnum,
) -> tonic::transport::server::Router {
    debug!("Adding service to router: {:?}", service);
    match service {
        ServiceEnum::Actuator(svc) => router.add_service(svc),
        ServiceEnum::Imu(svc) => router.add_service(svc),
        ServiceEnum::ProcessManager(svc) => router.add_service(svc),
    }
}

async fn run_server(
    platform: &(dyn Platform + Send + Sync),
    operations_service: Arc<OperationsServiceImpl>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let mut server_builder = Server::builder();

    let services = platform.create_services(operations_service.clone()).await?;

    let operations_service = OperationsServer::new(operations_service);

    let mut router = server_builder.add_service(operations_service);

    // Add remaining services using the helper function
    for service in services {
        router = add_service_to_router(router, service);
    }

    info!("Serving on {}", addr);
    // Serve the accumulated router
    router.serve(addr).await?;
    Ok(())
}

struct DaemonState {
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
    platform: PlatformImpl,
}

struct CompressedWriter {
    encoder: Option<GzEncoder<BufWriter<File>>>,
    path: PathBuf,
}

// TODO: The encoder doesn't close properly, so this needs to be fixed later.
impl CompressedWriter {
    fn new(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::create(path.as_ref())?;
        let buffered = BufWriter::new(file);
        Ok(Self {
            encoder: Some(GzEncoder::new(buffered, Compression::new(6))),
            path: path.as_ref().to_path_buf(),
        })
    }

    fn sync(&mut self) -> io::Result<()> {
        if let Some(encoder) = &mut self.encoder {
            encoder.flush()?;
            let buf_writer = encoder.get_mut();
            buf_writer.flush()?;
            let file = buf_writer.get_mut();
            file.sync_all()?;
        }
        Ok(())
    }

    fn finalize(&mut self) -> io::Result<()> {
        info!("Finalizing compressed log {}", self.path.display());
        if let Some(encoder) = self.encoder.take() {
            // Finish the compression
            let mut buf_writer = encoder.finish()?;
            // Flush the buffer
            buf_writer.flush()?;
            info!("Flushed compressed log {}", self.path.display());
            // Sync to disk
            buf_writer.get_mut().sync_all()?;
        }
        Ok(())
    }
}

impl Write for CompressedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(encoder) = &mut self.encoder {
            match encoder.write(buf) {
                Ok(size) => {
                    if size > 0 && buf.contains(&b'\n') {
                        self.sync()?;
                    }
                    Ok(size)
                }
                Err(e) => {
                    error!(
                        "Failed to write to compressed log {}: {}",
                        self.path.display(),
                        e
                    );
                    Err(e)
                }
            }
        } else {
            error!(
                "Attempted to write to finalized log {}",
                self.path.display()
            );
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Writer has been finalized",
            ))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sync()
    }
}

impl Drop for CompressedWriter {
    fn drop(&mut self) {
        if let Err(e) = self.finalize() {
            error!(
                "Failed to finalize compressed log {}: {}",
                self.path.display(),
                e
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // tracing
    let subscriber = tracing_subscriber::registry();

    // Always add stdout layer
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            EnvFilter::from_default_env()
                .add_directive("h2=error".parse().unwrap())
                .add_directive("grpc=error".parse().unwrap())
                .add_directive("rumqttc=error".parse().unwrap())
                .add_directive("kos_core::telemetry=error".parse().unwrap())
                .add_directive("polling=error".parse().unwrap())
                .add_directive("async_io=error".parse().unwrap())
                .add_directive("krec=error".parse().unwrap()),
        );

    let subscriber = subscriber.with(stdout_layer);

    let guard = if args.log {
        let log_dir = if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.data_local_dir().join("kos").join("logs")
        } else {
            PathBuf::from("~/.local/share/kos/logs")
        };

        std::fs::create_dir_all(&log_dir)?;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let temp_name = format!("kos-daemon_{}.log", timestamp);
        let final_name = format!("kos-daemon_{}.log.gz", timestamp);
        let log_path = log_dir.join(&final_name);

        info!("Writing compressed logs to: {}", log_path.display());

        let compressed_writer = CompressedWriter::new(&log_path)?;
        let (non_blocking, guard) = tracing_appender::non_blocking(compressed_writer);

        let file_layer = tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_writer(non_blocking)
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE);

        subscriber.with(file_layer).init();
        Some(guard)
    } else {
        subscriber.init();
        None
    };

    let mut state = DaemonState {
        _guard: guard,
        platform: PlatformImpl::new(),
    };

    // Setup signal handler
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        if let Ok(()) = signal::ctrl_c().await {
            let _ = shutdown_tx.send(());
        }
    });

    // telemetry
    Telemetry::initialize(
        format!("{}-{}", state.platform.name(), state.platform.serial()).as_str(),
        "localhost",
        1883,
    )
    .await?;

    let operations_store = Arc::new(Mutex::new(HashMap::new()));
    let operations_service = Arc::new(OperationsServiceImpl::new(operations_store));

    state.platform.initialize(operations_service.clone())?;

    tokio::select! {
        res = run_server(&state.platform, operations_service) => {
            if let Err(e) = res {
                error!("Server error: {:?}", e);
                std::process::exit(1);
            }
        }
        _ = shutdown_rx => {
            info!("Received shutdown signal, cleaning up...");
        }
    }

    Ok(())
}
