// TODO: Implement daemon for managing the robot.
// This will run the gRPC server and, if applicable, a runtime loop
// (e.g., actuator polling, loaded model inference).

use eyre::Result;
use kos_core::google_proto::longrunning::operations_server::OperationsServer;
use kos_core::hal::Operation;
use kos_core::services::OperationsServiceImpl;
use kos_core::Platform;
use kos_core::ServiceEnum;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;

#[cfg(feature = "sim")]
use sim::SimPlatform as PlatformImpl;

#[cfg(feature = "stub")]
use stub::StubPlatform as PlatformImpl;

fn add_service_to_router(
    router: tonic::transport::server::Router,
    service: ServiceEnum,
) -> tonic::transport::server::Router {
    debug!("Adding service to router: {:?}", service);
    match service {
        ServiceEnum::Actuator(svc) => router.add_service(svc),
        ServiceEnum::Imu(svc) => router.add_service(svc),
    }
}

async fn run_server(
    platform: &(dyn Platform + Send + Sync),
    operations_store: Arc<Mutex<HashMap<String, Operation>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut server_builder = Server::builder();

    let services = platform.create_services(operations_store.clone());

    let operations_service =
        OperationsServer::new(OperationsServiceImpl::new(operations_store.clone()));

    let mut router = server_builder.add_service(operations_service);

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
    // logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("h2=error".parse().unwrap())
                .add_directive("grpc=error".parse().unwrap())
        )
        .init();


    // long runningoperations store
    let operations_store = Arc::new(Mutex::new(HashMap::new()));

    // initialize platform
    let platform: Arc<Mutex<dyn Platform + Send + Sync>> =
        Arc::new(Mutex::new(PlatformImpl::new()));
    {
        let mut platform = platform.lock().await;
        info!("Initializing platform {}", platform.name());
        platform.initialize(operations_store.clone())?;
    }

    // run server
    let platform_clone = platform.clone();
    tokio::spawn(async move {
        let platform = platform_clone.lock().await;
        if let Err(e) = run_server(&*platform, operations_store).await {
            error!("Failed to run server: {:?}", e);
        }
    })
    .await?;

    Ok(())
}
