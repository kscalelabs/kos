use crate::{Arc, Operation, OperationsServiceImpl};
use async_trait::async_trait;
use eyre::{Result, WrapErr};
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorRequest},
    kos_proto::{actuator::*, common::{ActionResult, ErrorCode}, common::Error as KosError },
};
use std::collections::HashMap;

use robstride::{MotorType, MotorsSupervisor};

pub struct KBotActuator {
    motors_supervisor: MotorsSupervisor,
}

impl KBotActuator {
    pub fn new(
        _operations_service: Arc<OperationsServiceImpl>,
        port: &str,
        motor_infos: HashMap<u32, MotorType>,
        verbose: Option<bool>,
        max_update_rate: Option<u32>,
        zero_on_init: Option<bool>,
    ) -> Result<Self> {
        let motor_infos_u8 = motor_infos
            .into_iter()
            .map(|(k, v)| {
                let id =
                    u8::try_from(k).wrap_err_with(|| format!("Motor ID {} too large for u8", k))?;
                Ok((id, v))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        let motors_supervisor = MotorsSupervisor::new(
            port,
            &motor_infos_u8,
            verbose.unwrap_or(false),
            max_update_rate.unwrap_or(100000) as f64,
            zero_on_init.unwrap_or(false),
        )
        .map_err(|e| eyre::eyre!("Failed to create motors supervisor: {}", e))?;

        Ok(KBotActuator { motors_supervisor })
    }
}

#[async_trait]
impl Actuator for KBotActuator {
    async fn command_actuators(
        &self,
        commands: Vec<ActuatorCommand>,
    ) -> Result<Vec<ActionResult>> {
        let mut results = vec![];
        for command in commands {
            let mut motor_result = vec![];
            if let Some(position) = command.position {
                let result = self.motors_supervisor.set_position(
                    command.actuator_id as u8,
                    position.to_radians() as f32,
                );
                motor_result.push(result);
            }
            if let Some(velocity) = command.velocity {
                let result = self.motors_supervisor.set_velocity(
                    command.actuator_id as u8,
                    velocity as f32,
                );
                motor_result.push(result);
            }
            if let Some(torque) = command.torque {
                let result = self.motors_supervisor.set_torque(
                    command.actuator_id as u8,
                    torque as f32,
                );
                motor_result.push(result);
            }

            let success = motor_result.iter().all(|r| r.is_ok());
            let error = if !success {
                Some(KosError {
                    code: if motor_result.iter().any(|r| matches!(r, Err(e) if e.kind() == std::io::ErrorKind::NotFound)) {
                        ErrorCode::InvalidArgument as i32
                    } else {
                        ErrorCode::HardwareFailure as i32
                    },
                    message: motor_result
                        .iter()
                        .filter_map(|r| r.as_ref().err())
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join("; "),
                })
            } else {
                None
            };

            results.push(ActionResult {
                actuator_id: command.actuator_id,
                success,
                error,
            });
        }
        Ok(results)
    }

    async fn configure_actuator(
        &self,
        config: ConfigureActuatorRequest,
    ) -> Result<ActionResponse> {
        let motor_id = config.actuator_id as u8;
        let mut results = vec![];

        // Configure KP if provided
        if let Some(kp) = config.kp {
            let result = self.motors_supervisor.set_kp(motor_id, kp as f32);
            results.push(result);
        }

        // Configure KD if provided
        if let Some(kd) = config.kd {
            let result = self.motors_supervisor.set_kd(motor_id, kd as f32);
            results.push(result);
        }

        let success = results.iter().all(|r| r.is_ok());
        let error = if !success {
            Some(kos_core::kos_proto::common::Error {
                code: if results.iter().any(|r| matches!(r, Err(e) if e.kind() == std::io::ErrorKind::NotFound)) {
                    ErrorCode::InvalidArgument as i32
                } else {
                    ErrorCode::HardwareFailure as i32
                },
                message: results
                    .iter()
                    .filter_map(|r| r.as_ref().err())
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; "),
            })
        } else {
            None
        };

        Ok(ActionResponse { success, error })
    }

    async fn calibrate_actuator(&self, _request: CalibrateActuatorRequest) -> Result<Operation> {
        Ok(Operation::default())
    }

    async fn get_actuators_state(
        &self,
        _actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>> {
        let feedback = self.motors_supervisor.get_latest_feedback();
        Ok(feedback
            .iter()
            .map(|(id, state)| ActuatorStateResponse {
                actuator_id: u32::from(*id),
                online: matches!(state.mode, robstride::MotorMode::Motor),
                position: Some(state.position.to_degrees() as f64),
                velocity: Some(state.velocity as f64),
                torque: Some(state.torque as f64),
                temperature: None,
                voltage: None,
                current: None,
            })
            .collect())
    }
}
