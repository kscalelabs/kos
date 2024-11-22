mod actuator;
mod imu;
mod process_manager;
pub use actuator::*;
pub use imu::*;
pub use process_manager::*;

use eyre::Result;
use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
    process_manager::process_manager_service_server::ProcessManagerServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl, ProcessManagerServiceImpl};
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
            ServiceEnum::ProcessManager(ProcessManagerServiceServer::new(
                ProcessManagerServiceImpl::new(Arc::new(StubProcessManager::new())),
            )),
        ])
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
