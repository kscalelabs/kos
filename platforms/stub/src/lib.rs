mod actuator;
mod imu;

pub use actuator::*;
pub use imu::*;

use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl};
use kos_core::{Platform, ServiceEnum};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    fn initialize(&mut self, operations_store: Arc<Mutex<HashMap<String, Operation>>>) -> eyre::Result<()> {
        // Start the supervisor task
        StubActuator::start_supervisor_task(operations_store.clone());
        Ok(())
    }

    fn create_services(
        &self,
        operations_store: Arc<Mutex<HashMap<String, Operation>>>,
    ) -> Vec<ServiceEnum> {
        // Add available services here
        vec![
            ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(
                StubIMU::new(operations_store.clone()),
            )))),
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(StubActuator::new(operations_store.clone())),
            ))),
        ]
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
