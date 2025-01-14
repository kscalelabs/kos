use kos::kos_runtime;

#[tokio::test]
async fn test_stub_process_manager() {
    let platform = StubPlatform::new();
    kos_runtime(platform).await.unwrap();
    assert!(true);
}
