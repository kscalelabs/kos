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
                let process_manager =
                    KBotProcessManager::new(self.name().to_string(), self.serial())
                        .wrap_err("Failed to initialize GStreamer process manager")?;

                let actuator =
                    KBotActuator::new(
                        operations_service,
                        vec!["can0"],
                        Duration::from_secs(1),
                        Duration::from_nanos(3_333_333),
                        &[(1, robstridev2::ActuatorType::RobStride04),
                        (1, ActuatorType::RobStride04),  // Left Hip
                        (2, robstridev2::ActuatorType::RobStride03),  // Left Knee
                        (3, robstridev2::ActuatorType::RobStride03),  // Right Hip
                        (4, robstridev2::ActuatorType::RobStride04),  // Right Knee
                        (5, robstridev2::ActuatorType::RobStride02),  // Torso
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
                let actuator =
                    KBotActuator::new(
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
