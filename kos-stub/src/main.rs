use kos::daemon::kos_runtime;
use kos_stub::StubPlatform;

#[tokio::main]
async fn main() {
    let platform = Box::new(StubPlatform::new());
    kos_runtime(platform).await.unwrap();
}
