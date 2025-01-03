// TODO: Implement daemon for managing the robot.
// This will run the gRPC server and, if applicable, a runtime loop
// (e.g., actuator polling, loaded model inference).

use clap::Parser;
use eyre::Result;
use kos_core::google_proto::longrunning::operations_server::OperationsServer;
use kos_core::services::OperationsServiceImpl;
use kos_core::telemetry::Telemetry;
use kos_core::Platform;
use kos_core::ServiceEnum;
use std::collections::HashMap;
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
use kos_zeroth_01::ZBotPlatform as PlatformImpl;

#[cfg(feature = "kos-kbot")]
use kos_kbot::KbotPlatform as PlatformImpl;

mod file_logging;
use file_logging::{setup_logging, cleanup_logging};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable file logging
    #[arg(long, default_value_t = false)]
    log: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
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

    let _subscriber = subscriber.with(stdout_layer);

    let guard = setup_logging(args.log, &args.log_level)?;

    let mut state = DaemonState {
        _guard: guard,
        platform: PlatformImpl::new(),
    };

    // Setup signal handler
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

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
            cleanup_logging(state._guard.take());
        }
    }

    Ok(())
}
