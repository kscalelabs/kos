mod actuator;
mod process_manager;

#[cfg(target_os = "linux")]
mod hexmove;

pub use actuator::*;
pub use robstride::{ActuatorConfiguration, ActuatorType};

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
                        // "can0",
                        "can1", "can2",
                    ],
                    Duration::from_secs(1),
                    // Duration::from_nanos(3_333_333),
                    Duration::from_millis(7),
                    &[
                        // Left Arm
                        (
                            11,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            12,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            13,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            14,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            15,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            16,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride00,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        // Right Arm
                        (
                            21,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            22,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            23,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            24,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            25,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            26,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride00,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        // Left Leg
                        (
                            31,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride04,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            32,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            33,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            34,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride04,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            35,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        // Right Leg
                        (
                            41,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride04,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            42,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            43,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride03,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            44,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride04,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
                        (
                            45,
                            ActuatorConfiguration {
                                actuator_type: ActuatorType::RobStride02,
                                max_angle_change: Some(4.0f32.to_radians()),
                                max_velocity: Some(10.0f32.to_radians()),
                            },
                        ),
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
                    &[(
                        1,
                        robstride::ActuatorConfiguration {
                            actuator_type: ActuatorType::RobStride04,
                            max_angle_change: Some(2.0f32.to_radians()),
                            max_velocity: Some(10.0f32.to_radians()),
                        },
                    )],
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
