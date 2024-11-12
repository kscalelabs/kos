// TODO: Implement daemon.
// This will be the main process that will manage the robot.
// It will run the gRPC server, and, if applicable,
// the runtime loop (e.g. actuator polling, loaded model inference).

use eyre::Result;

use kos_core::kos_proto::imu::imu_service_server::ImuServiceServer;
use kos_core::services::*;
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (imu) = {
        #[cfg(feature = "sim")]
        {
            (sim::SimIMU::new())
        }

        #[cfg(feature = "stub")]
        {
            (stub::StubIMU::new())
        }
    };

    let imu_service = IMUServiceImpl::new(Arc::new(imu));

    // Add additional services here
    // let another_service = AnotherService::new(...);

    let addr = "0.0.0.0:50051".parse()?;

    println!("Starting gRPC at {}", addr);

    Server::builder()
        .add_service(ImuServiceServer::new(imu_service))
        .serve(addr)
        .await?;

    Ok(())
}
