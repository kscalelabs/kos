// TODO: Implement daemon for managing the robot.
// This will run the gRPC server and, if applicable, a runtime loop
// (e.g., actuator polling, loaded model inference).

use eyre::Result;
use kos_core::Platform;
use kos_core::ServiceEnum;
use std::sync::Arc;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower::ServiceBuilder;
use tokio::sync::Mutex; // Use Tokio's async-aware Mutex

#[cfg(feature = "sim")]
use sim::SimPlatform as PlatformImpl;

#[cfg(feature = "stub")]
use stub::StubPlatform as PlatformImpl;

// Helper function to add a service to an existing router
fn add_service_to_router(
    router: tonic::transport::server::Router,
    service: ServiceEnum,
) -> tonic::transport::server::Router {
    match service {
        ServiceEnum::Actuator(svc) => router.add_service(
            ServiceBuilder::new()
                .layer(GrpcWebLayer::new())
                .service(svc),
        ),
        ServiceEnum::Imu(svc) => router.add_service(
            ServiceBuilder::new()
                .layer(GrpcWebLayer::new())
                .service(svc),
        ),
        // Add more service cases here as needed
    }
}

async fn run_server(
    platform: &(dyn Platform + Send + Sync),
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut server_builder = Server::builder();

    let mut services = platform.get_services();
    // Initialize the router with the first service
    let first_service = services.pop();
    let mut router = match first_service {
        Some(ServiceEnum::Actuator(svc)) => server_builder.add_service(
            ServiceBuilder::new()
                .layer(GrpcWebLayer::new())
                .service(svc),
        ),
        Some(ServiceEnum::Imu(svc)) => server_builder.add_service(
            ServiceBuilder::new()
                .layer(GrpcWebLayer::new())
                .service(svc),
        ),
        None => return Ok(()), // No services to add, exit early
    };

    // Add remaining services using the helper function
    for service in services {
        router = add_service_to_router(router, service);
    }

    // Serve the accumulated router
    router.serve(addr).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let platform: Arc<Mutex<dyn Platform + Send + Sync>> = Arc::new(Mutex::new(PlatformImpl::new()));

    {
        let mut platform = platform.lock().await;
        tracing::info!("Initializing platform {}", platform.name());
        platform.initialize()?;
    }

    // Use the run_server function in a separate task
    let platform_clone = platform.clone();
    tokio::spawn(async move {
        let platform = platform_clone.lock().await;
        run_server(&*platform).await.unwrap();
    }).await?;

    Ok(())
}
