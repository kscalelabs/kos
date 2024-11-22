mod actuator;
mod process_manager;

#[cfg(target_os = "linux")]
mod hexmove;

pub use actuator::*;

#[cfg(target_os = "linux")]
pub use hexmove::*;
pub use process_manager::*;

use eyre::{Result, WrapErr};
use kos_core::hal::Operation;
use kos_core::kos_proto::actuator::actuator_service_server::ActuatorServiceServer;
use kos_core::kos_proto::process_manager::process_manager_service_server::ProcessManagerServiceServer;
use kos_core::{
    services::{ActuatorServiceImpl, OperationsServiceImpl, ProcessManagerServiceImpl},
    Platform, ServiceEnum,
};
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
        // TODO: Get the serial number from the device
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
        if cfg!(target_os = "linux") {
            // Create the process manager first and handle any errors
            let process_manager = KBotProcessManager::new(self.name().to_string(), self.serial())
                .wrap_err("Failed to initialize GStreamer process manager")?;

            Ok(vec![
                ServiceEnum::Imu(
                    kos_core::kos_proto::imu::imu_service_server::ImuServiceServer::new(
                        kos_core::services::IMUServiceImpl::new(Arc::new(
                            KBotIMU::new(operations_service.clone(), "can0", 1, 1)
                                .wrap_err("Failed to create IMU")?,
                        )),
                    ),
                ),
                // ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                //     Arc::new(
                //         KBotActuator::new(
                //             operations_service,
                //             "/dev/ttyCH341USB0",
                //             HashMap::from([
                //                 (1, MotorType::Type04),
                //                 (2, MotorType::Type04),
                //                 (3, MotorType::Type04),
                //                 (4, MotorType::Type04),
                //                 (5, MotorType::Type04),
                //                 (6, MotorType::Type01),
                //             ]),
                //             None,
                //             None,
                //             None,
                //         )
                //         .wrap_err("Failed to create actuator")?,
                //     ),
                // ))),
                ServiceEnum::ProcessManager(ProcessManagerServiceServer::new(
                    ProcessManagerServiceImpl::new(Arc::new(process_manager)),
                )),
            ])
        } else {
            Ok(vec![ServiceEnum::Actuator(ActuatorServiceServer::new(
                ActuatorServiceImpl::new(Arc::new(
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
                )),
            ))])
        }
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
