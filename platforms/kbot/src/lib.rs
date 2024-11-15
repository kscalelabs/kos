mod actuator;

#[cfg(target_os = "linux")]
mod hexmove;

pub use actuator::*;

#[cfg(target_os = "linux")]
pub use hexmove::*;

use eyre::{Result, WrapErr};
use kos_core::hal::Operation;
use kos_core::kos_proto::{
    actuator::actuator_service_server::ActuatorServiceServer,
    imu::imu_service_server::ImuServiceServer,
};
use kos_core::services::{ActuatorServiceImpl, IMUServiceImpl};
use kos_core::{services::OperationsServiceImpl, Platform, ServiceEnum};
use robstride::MotorType;
use std::collections::HashMap;
use std::sync::Arc;

pub struct KbotPlatform {}

impl KbotPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for KbotPlatform {
    fn default() -> Self {
        KbotPlatform::new()
    }
}

impl Platform for KbotPlatform {
    fn name(&self) -> &'static str {
        "KBot"
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
        #[cfg(target_os = "linux")]
        Ok(vec![
            ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(
                KBotIMU::new(operations_service.clone(), "can0", 1, 1)
                    .wrap_err("Failed to create IMU")?,
            )))),
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(
                    KBotActuator::new(
                        operations_service,
                        "/dev/ttyCH341USB0",
                        HashMap::from([
                            (1, MotorType::Type04),
                            (2, MotorType::Type04),
                            (3, MotorType::Type04),
                            (4, MotorType::Type04),
                            (5, MotorType::Type04),
                            (6, MotorType::Type01),
                        ]),
                        None,
                        None,
                        None,
                    )
                    .wrap_err("Failed to create actuator")?,
                ),
            ))),
        ]);

        #[cfg(not(target_os = "linux"))]
        Ok(vec![
            ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                Arc::new(
                    KBotActuator::new(
                    operations_service,
                    "/dev/ttyCH341USB0",
                    HashMap::from([
                        (1, MotorType::Type04),
                        (2, MotorType::Type04),
                        (3, MotorType::Type04),
                        (4, MotorType::Type04),
                        (5, MotorType::Type04),
                        (6, MotorType::Type01),
                    ]),
                    None,
                    None,
                    None,
                )
                .wrap_err("Failed to create actuator")?,
                ),
            ))),
        ])
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
