mod actuator;
mod imu;
mod process_manager;
pub use actuator::*;
pub use imu::*;
pub use process_manager::*;

use async_trait::async_trait;
use eyre::Result;
use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
    process_manager::process_manager_service_server::ProcessManagerServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl, ProcessManagerServiceImpl};
use kos_core::{services::OperationsServiceImpl, Platform, ServiceEnum};
use std::future::Future;
use std::pin::Pin;
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

#[async_trait]
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

    fn create_services<'a>(
        &'a self,
        _operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(vec![]) // or whatever the stub implementation should return
        })
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
