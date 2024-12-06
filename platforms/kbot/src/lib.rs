mod actuator;
mod process_manager;

#[cfg(target_os = "linux")]
mod hexmove;

pub use actuator::*;
pub use robstridev2::ActuatorType;

#[cfg(target_os = "linux")]
pub use hexmove::*;
pub use process_manager::*;

use async_trait::async_trait;
use eyre::WrapErr;
use kos_core::hal::Operation;
use kos_core::kos_proto::actuator::actuator_service_server::ActuatorServiceServer;
use kos_core::kos_proto::process_manager::process_manager_service_server::ProcessManagerServiceServer;
use kos_core::{
    services::{ActuatorServiceImpl, OperationsServiceImpl, ProcessManagerServiceImpl},
    Platform, ServiceEnum,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

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

#[async_trait]
impl Platform for KbotPlatform {
    fn name(&self) -> &'static str {
        "KBot"
    }

    fn serial(&self) -> String {
        // TODO: Get the serial number from the device
        "00000000".to_string()
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        // Initialize the platform
        Ok(())
    }

    fn create_services<'a>(
        &'a self,
        operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>> {
        Box::pin(async move {
            if cfg!(target_os = "linux") {
                tracing::debug!("Initializing KBot services for Linux");

                let process_manager =
                    KBotProcessManager::new(self.name().to_string(), self.serial())
                        .wrap_err("Failed to initialize GStreamer process manager")?;

                let actuator = KBotActuator::new(
                    operations_service,
                    vec![
                        // "/dev/ttyCH341USB0",
                        // "/dev/ttyCH341USB1",
                        // "/dev/ttyCH341USB2",
                        // "/dev/ttyCH341USB3",
                        "can0", "can1", "can2",
                    ],
                    Duration::from_secs(1),
                    // Duration::from_nanos(3_333_333),
                    Duration::from_millis(7),
                    &[
                        // Left Arm
                        (11, ActuatorType::RobStride03),
                        (12, ActuatorType::RobStride03),
                        (13, ActuatorType::RobStride02),
                        (14, ActuatorType::RobStride02),
                        (15, ActuatorType::RobStride02),
                        (16, ActuatorType::RobStride00),
                        // Right Arm
                        (21, ActuatorType::RobStride03),
                        (22, ActuatorType::RobStride03),
                        (23, ActuatorType::RobStride02),
                        (24, ActuatorType::RobStride02),
                        (25, ActuatorType::RobStride02),
                        (26, ActuatorType::RobStride00),
                        // Left Leg
                        (31, ActuatorType::RobStride04),
                        (32, ActuatorType::RobStride03),
                        (33, ActuatorType::RobStride03),
                        (34, ActuatorType::RobStride04),
                        (35, ActuatorType::RobStride02),
                        // Right Leg
                        (41, ActuatorType::RobStride04),
                        (42, ActuatorType::RobStride03),
                        (43, ActuatorType::RobStride03),
                        (44, ActuatorType::RobStride04),
                        (45, ActuatorType::RobStride02),
                    ],
                )
                .await
                .wrap_err("Failed to create actuator")?;

                Ok(vec![
                    ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                        Arc::new(actuator),
                    ))),
                    ServiceEnum::ProcessManager(ProcessManagerServiceServer::new(
                        ProcessManagerServiceImpl::new(Arc::new(process_manager)),
                    )),
                ])
            } else {
                let actuator = KBotActuator::new(
                    operations_service,
                    vec!["can0"],
                    Duration::from_secs(1),
                    Duration::from_nanos(3_333_333),
                    &[(1, robstridev2::ActuatorType::RobStride04)],
                )
                .await
                .wrap_err("Failed to create actuator")?;

                Ok(vec![ServiceEnum::Actuator(ActuatorServiceServer::new(
                    ActuatorServiceImpl::new(Arc::new(actuator)),
                ))])
            }
        })
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
