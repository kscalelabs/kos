// TODO: Implement daemon for managing the robot.
// This will run the gRPC server and, if applicable, a runtime loop
// (e.g., actuator polling, loaded model inference).

use eyre::Result;
use kos_core::google_proto::longrunning::operations_server::OperationsServer;
use kos_core::services::OperationsServiceImpl;
use kos_core::telemetry::Telemetry;
use kos_core::Platform;
use kos_core::ServiceEnum;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;

#[cfg(not(any(feature = "kos-sim", feature = "kos-zeroth-01", feature = "kos-kbot")))]
use kos_stub::StubPlatform as PlatformImpl;

#[cfg(feature = "kos-sim")]
use kos_sim::SimPlatform as PlatformImpl;

#[cfg(feature = "kos-zeroth-01")]
use kos_zeroth_01::Zeroth01Platform as PlatformImpl;

#[cfg(feature = "kos-kbot")]
use kos_kbot::KbotPlatform as PlatformImpl;

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

#[tokio::main]
async fn main() -> Result<()> {
    // logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("h2=error".parse().unwrap())
                .add_directive("grpc=error".parse().unwrap())
                .add_directive("rumqttc=error".parse().unwrap())
                .add_directive("kos_core::telemetry=error".parse().unwrap())
                .add_directive("polling=error".parse().unwrap())
                .add_directive("async_io=error".parse().unwrap())
                .add_directive("krec=error".parse().unwrap()),
        )
        .init();

    let mut platform = PlatformImpl::new();

    // telemetry
    Telemetry::initialize(
        format!("{}-{}", platform.name(), platform.serial()).as_str(),
        "localhost",
        1883,
    )
    .await?;

    let operations_store = Arc::new(Mutex::new(HashMap::new()));
    let operations_service = Arc::new(OperationsServiceImpl::new(operations_store));

    platform.initialize(operations_service.clone())?;

    if let Err(e) = run_server(&platform, operations_service).await {
        error!("Server error: {:?}", e);
        std::process::exit(1);
    }

    Ok(())
}
