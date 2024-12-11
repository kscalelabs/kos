use chrono::Local;
use directories::BaseDirs;
use eyre::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, Layer};

pub struct CompressedWriter {
    encoder: Option<GzEncoder<BufWriter<File>>>,
    path: PathBuf,
}

impl CompressedWriter {
    pub fn new(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
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

    pub fn finalize(&mut self) -> io::Result<()> {
        if let Some(encoder) = self.encoder.take() {
            info!("Finalizing compressed log {}", self.path.display());
            // Finish the compression and get the BufWriter back
            let mut buf_writer = encoder.finish()?;
            // Flush the buffer
            buf_writer.flush()?;
            // Sync to disk
            buf_writer.get_mut().sync_all()?;
            info!("Successfully finalized log {}", self.path.display());
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

pub fn setup_logging(
    enable_file_logging: bool,
    log_level: &str,
) -> Result<Option<tracing_appender::non_blocking::WorkerGuard>> {
    let level_filter = match log_level.to_lowercase().as_str() {
        "trace" => tracing_subscriber::filter::LevelFilter::TRACE,
        "debug" => tracing_subscriber::filter::LevelFilter::DEBUG,
        "info" => tracing_subscriber::filter::LevelFilter::INFO,
        "warn" => tracing_subscriber::filter::LevelFilter::WARN,
        "error" => tracing_subscriber::filter::LevelFilter::ERROR,
        _ => {
            eprintln!("Invalid log level '{}', defaulting to 'info'", log_level);
            tracing_subscriber::filter::LevelFilter::INFO
        }
    };

    let subscriber = tracing_subscriber::registry();

    // Update stdout layer to use the specified level
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            EnvFilter::from_default_env()
                .add_directive(format!("kos={}", log_level).parse().unwrap())
                .add_directive("h2=error".parse().unwrap())
                .add_directive("grpc=error".parse().unwrap())
                .add_directive("rumqttc=error".parse().unwrap())
                .add_directive("kos_core::telemetry=error".parse().unwrap())
                .add_directive("polling=error".parse().unwrap())
                .add_directive("async_io=error".parse().unwrap())
                .add_directive("krec=error".parse().unwrap()),
        );

    let subscriber = subscriber.with(stdout_layer);

    if enable_file_logging {
        let log_dir = if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.data_local_dir().join("kos").join("logs")
        } else {
            PathBuf::from("~/.local/share/kos/logs")
        };

        std::fs::create_dir_all(&log_dir)?;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let final_name = format!("kos-daemon_{}.log.gz", timestamp);
        let log_path = log_dir.join(&final_name);

        let compressed_writer = CompressedWriter::new(&log_path)?;
        let (non_blocking, guard) = tracing_appender::non_blocking(compressed_writer);

        let file_layer = tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_writer(non_blocking)
            .with_filter(level_filter);

        subscriber.with(file_layer).init();
        Ok(Some(guard))
    } else {
        subscriber.init();
        Ok(None)
    }
}

pub fn cleanup_logging(guard: Option<tracing_appender::non_blocking::WorkerGuard>) {
    if let Some(guard) = guard {
        // Ensure we flush any pending writes before dropping
        drop(guard);
        // Give a small amount of time for the worker to finish
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
