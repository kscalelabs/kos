mod actuator;
mod imu;

pub use actuator::*;
pub use imu::*;

use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl};
use kos_core::{Platform, ServiceEnum};
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

    fn initialize(&mut self) -> eyre::Result<()> {
        Ok(())
    }

    fn get_services(&self) -> Vec<ServiceEnum> {
        vec![
            ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(
                StubIMU::new(),
            )))),
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(StubActuator::new()),
            ))),
        ]
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        Ok(())
    }
}
