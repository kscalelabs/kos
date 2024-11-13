mod actuator;
mod hexmove;

pub use actuator::*;
pub use hexmove::*;

use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl};
use kos_core::{services::OperationsServiceImpl, Platform, ServiceEnum};
use std::sync::Arc;

pub struct KscaleProPlatform {}

impl KscaleProPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for KscaleProPlatform {
    fn default() -> Self {
        KscaleProPlatform::new()
    }
}

impl Platform for KscaleProPlatform {
    fn name(&self) -> &'static str {
        "Kscale Pro"
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        // Initialize the platform
        Ok(())
    }

    fn create_services(&self, operations_service: Arc<OperationsServiceImpl>) -> Vec<ServiceEnum> {
        // Add available services here
        vec![
            ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(
                KscaleProIMU::new("can0", 1, 1),
            )))),
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(KscaleProActuator::new(
                    "/dev/ttyCH341USB0",
                    HashMap::new()
                )),
            ))),
        ]
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
