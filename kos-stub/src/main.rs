use kos::daemon::kos_runtime;
use kos_stub::StubPlatform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let platform = Box::new(StubPlatform::new());
    kos_runtime(platform).await.map_err(|e| {
        eprintln!("Runtime error: {}", e);
        e
    })?;
    Ok(())
}
