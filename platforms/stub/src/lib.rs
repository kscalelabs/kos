mod actuator;
mod imu;

pub use actuator::*;
pub use imu::*;

use eyre::Result;
use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl};
use kos_core::{services::OperationsServiceImpl, Platform, ServiceEnum};
use std::sync::Arc;

pub struct StubPlatform {}

impl StubPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StubPlatform {
    fn default() -> Self {
        StubPlatform::new()
    }
}

impl Platform for StubPlatform {
    fn name(&self) -> &'static str {
        "Stub"
    }

    fn serial(&self) -> String {
        "00000000".to_string()
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        // Initialize the platform
        Ok(())
    }

    fn create_services(
        &self,
        operations_service: Arc<OperationsServiceImpl>,
    ) -> Result<Vec<ServiceEnum>> {
        // Add available services here
        Ok(vec![
            ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(
                StubIMU::new(operations_service.clone()),
            )))),
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(StubActuator::new(operations_service.clone())),
            ))),
        ])
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
