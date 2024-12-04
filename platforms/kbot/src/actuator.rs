use crate::{Arc, Operation, OperationsServiceImpl};
use async_trait::async_trait;
use eyre::Result;
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorRequest},
    kos_proto::{
        actuator::*,
        common::Error as KosError,
        common::{ActionResult, ErrorCode},
    },
};

use robstridev2::{CH341Transport, ControlConfig, SocketCanTransport, Supervisor, TransportType};
use std::time::Duration;
use tokio::sync::Mutex;

pub struct KBotActuator {
    supervisor: Arc<Mutex<Supervisor>>,
}

impl KBotActuator {
    pub async fn new(
        _operations_service: Arc<OperationsServiceImpl>,
        ports: Vec<&str>,
        actuator_timeout: Duration,
        polling_interval: Duration,
        desired_actuator_types: &[(u8, robstridev2::ActuatorType)],
    ) -> Result<Self> {
        let mut supervisor = Supervisor::new(actuator_timeout)?;
        let mut found_motors = vec![false; desired_actuator_types.len()];

        for port in ports.clone() {
            if port.starts_with("/dev/tty") {
                let serial = CH341Transport::new(port.to_string()).await?;
                supervisor
                    .add_transport(format!("{}", port), TransportType::CH341(serial))
                    .await?;
            } else if port.starts_with("can") {
                let can = SocketCanTransport::new(port.to_string()).await?;
                supervisor
                    .add_transport(format!("{}", port), TransportType::SocketCAN(can))
                    .await?;
            } else {
                return Err(eyre::eyre!("Invalid port: {}", port));
            }
        }

        let mut supervisor_runner = supervisor.clone_controller();
        let _supervisor_handle = tokio::spawn(async move {
            if let Err(e) = supervisor_runner.run(polling_interval).await {
                tracing::error!("Supervisor task failed: {}", e);
            }
        });

        for port in ports.clone() {
            let discovered_ids = supervisor
                .scan_bus(0xFD, port, desired_actuator_types)
                .await?;
            
            for (idx, (motor_id, _)) in desired_actuator_types.iter().enumerate() {
                if discovered_ids.contains(motor_id) {
                    found_motors[idx] = true;
                }
            }
        }

        for (idx, (motor_id, motor_type)) in desired_actuator_types.iter().enumerate() {
            if !found_motors[idx] {
                tracing::warn!(
                    "Configured motor not found - ID: {}, Type: {:?}",
                    motor_id,
                    motor_type
                );
            }
        }

        Ok(KBotActuator {
            supervisor: Arc::new(Mutex::new(supervisor)),
        })
    }
}

#[async_trait]
impl Actuator for KBotActuator {
    async fn command_actuators(&self, commands: Vec<ActuatorCommand>) -> Result<Vec<ActionResult>> {
        let mut results = vec![];
        for command in commands {
            let motor_id = command.actuator_id as u8;
            let mut supervisor = self.supervisor.lock().await;
            let result = supervisor
                .command(
                    motor_id,
                    command
                        .position
                        .map(|p| p.to_radians() as f32)
                        .unwrap_or(0.0),
                    command
                        .velocity
                        .map(|v| v.to_radians() as f32)
                        .unwrap_or(0.0),
                    command.torque.map(|t| t as f32).unwrap_or(0.0),
                )
                .await;

            let success = result.is_ok();
            let error = result.err().map(|e| KosError {
                code: ErrorCode::HardwareFailure as i32,
                message: e.to_string(),
            });

            results.push(ActionResult {
                actuator_id: command.actuator_id,
                success,
                error,
            });
        }
        Ok(results)
    }

    async fn configure_actuator(&self, config: ConfigureActuatorRequest) -> Result<ActionResponse> {
        let motor_id = config.actuator_id as u8;

        let control_config = ControlConfig {
            kp: config.kp.unwrap_or(0.0) as f32,
            kd: config.kd.unwrap_or(0.0) as f32,
            max_torque: Some(2.0),
            max_velocity: Some(5.0),
            max_current: Some(10.0),
        };

        let mut supervisor = self.supervisor.lock().await;
        let result = supervisor.configure(motor_id, control_config).await;

        if let Some(torque_enabled) = config.torque_enabled {
            if torque_enabled {
                supervisor.enable(motor_id).await?;
            } else {
                supervisor.disable(motor_id, true).await?;
            }
        }

        if let Some(zero_position) = config.zero_position {
            if zero_position {
                supervisor.zero(motor_id).await?;
            }
        }

        if let Some(new_actuator_id) = config.new_actuator_id {
            supervisor
                .change_id(motor_id, new_actuator_id as u8)
                .await?;
        }

        let success = result.is_ok();
        let error = result.err().map(|e| KosError {
            code: ErrorCode::HardwareFailure as i32,
            message: e.to_string(),
        });

        Ok(ActionResponse { success, error })
    }

    async fn calibrate_actuator(&self, _request: CalibrateActuatorRequest) -> Result<Operation> {
        Ok(Operation::default())
    }

    async fn get_actuators_state(
        &self,
        actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>> {
        let mut responses = vec![];
        for id in actuator_ids {
            let supervisor = self.supervisor.lock().await;
            if let Ok(Some((feedback, ts))) = supervisor.get_feedback(id as u8).await {
                responses.push(ActuatorStateResponse {
                    actuator_id: id,
                    online: ts.elapsed().unwrap_or(Duration::from_secs(1)) < Duration::from_secs(1),
                    position: Some(feedback.angle.to_degrees() as f64),
                    velocity: Some(feedback.velocity.to_degrees() as f64),
                    torque: Some(feedback.torque as f64),
                    temperature: Some(feedback.temperature as f64),
                    voltage: None,
                    current: None,
                });
            }
        }
        Ok(responses)
    }
}
