use crate::{Arc, Operation, OperationsServiceImpl};
use async_trait::async_trait;
use eyre::{Result, WrapErr};
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorRequest},
    kos_proto::{
        actuator::*,
        common::Error as KosError,
        common::{ActionResult, ErrorCode},
    },
};
use std::collections::HashMap;

use robstride::{MotorType, MotorsSupervisor};

pub struct KBotActuator {
    motors_supervisors: HashMap<String, MotorsSupervisor>,
    motor_id_map: HashMap<u32, (String, u8)>,
}

impl KBotActuator {
    pub fn new(
        _operations_service: Arc<OperationsServiceImpl>,
        port_motor_map: HashMap<&str, HashMap<u32, MotorType>>,
        verbose: Option<bool>,
        max_update_rate: Option<u32>,
        zero_on_init: Option<bool>,
    ) -> Result<Self> {
        let mut motor_id_map = HashMap::new();
        
        let motors_supervisors = port_motor_map
            .into_iter()
            .map(|(port, motor_infos)| {
                for motor_id in motor_infos.keys() {
                    if motor_id_map.insert(*motor_id, (port.to_string(), *motor_id as u8)).is_some() {
                        return Err(eyre::eyre!("Duplicate motor ID: {}", motor_id));
                    }
                }

                let motor_infos_u8 = motor_infos
                    .into_iter()
                    .map(|(k, v)| {
                        let id = u8::try_from(k)
                            .wrap_err_with(|| format!("Motor ID {} too large for u8", k))?;
                        Ok((id, v))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;

                let supervisor = MotorsSupervisor::new(
                    port,
                    &motor_infos_u8,
                    verbose.unwrap_or(false),
                    max_update_rate.unwrap_or(100000) as f64,
                    zero_on_init.unwrap_or(false),
                )
                .map_err(|e| eyre::eyre!("Failed to create motors supervisor for port {}: {}", port, e))?;

                Ok((port.to_string(), supervisor))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(KBotActuator { 
            motors_supervisors,
            motor_id_map,
        })
    }

    pub fn get_supervisor_for_motor(&self, motor_id: u32) -> Result<(&MotorsSupervisor, u8)> {
        let (port, local_id) = self.motor_id_map
            .get(&motor_id)
            .ok_or_else(|| eyre::eyre!("Motor ID {} not found", motor_id))?;
            
        let supervisor = self.motors_supervisors
            .get(port)
            .ok_or_else(|| eyre::eyre!("Supervisor for port {} not found", port))?;

        Ok((supervisor, *local_id))
    }
}

#[async_trait]
impl Actuator for KBotActuator {
    async fn command_actuators(&self, commands: Vec<ActuatorCommand>) -> Result<Vec<ActionResult>> {
        let mut results = vec![];
        for command in commands {
            let motor_id = command.actuator_id as u32;
            let (supervisor, local_id) = match self.get_supervisor_for_motor(motor_id) {
                Ok(supervisor_info) => supervisor_info,
                Err(e) => {
                    results.push(ActionResult {
                        actuator_id: command.actuator_id,
                        success: false,
                        error: Some(KosError {
                            code: ErrorCode::InvalidArgument as i32,
                            message: e.to_string(),
                        }),
                    });
                    continue;
                }
            };

            let mut motor_result = vec![];
            if let Some(position) = command.position {
                let result = supervisor.set_position(local_id, position.to_radians() as f32);
                motor_result.push(result);
            }
            if let Some(velocity) = command.velocity {
                let result = supervisor.set_velocity(local_id, velocity as f32);
                motor_result.push(result);
            }
            if let Some(torque) = command.torque {
                let result = supervisor.set_torque(local_id, torque as f32);
                motor_result.push(result);
            }

            let success = motor_result.iter().all(|r| r.is_ok());
            let error = if !success {
                Some(KosError {
                    code: if motor_result
                        .iter()
                        .any(|r| matches!(r, Err(e) if e.kind() == std::io::ErrorKind::NotFound))
                    {
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

    async fn configure_actuator(&self, config: ConfigureActuatorRequest) -> Result<ActionResponse> {
        let (supervisor, local_id) = match self.get_supervisor_for_motor(config.actuator_id as u32) {
            Ok(supervisor_info) => supervisor_info,
            Err(e) => return Ok(ActionResponse {
                success: false,
                error: Some(KosError {
                    code: ErrorCode::InvalidArgument as i32,
                    message: e.to_string(),
                }),
            }),
        };

        let mut results = vec![];
        if let Some(kp) = config.kp {
            let result = supervisor.set_kp(local_id, kp as f32);
            results.push(result);
        }
        if let Some(kd) = config.kd {
            let result = supervisor.set_kd(local_id, kd as f32);
            results.push(result);
        }

        let success = results.iter().all(|r| r.is_ok());
        let error = if !success {
            Some(kos_core::kos_proto::common::Error {
                code: if results
                    .iter()
                    .any(|r| matches!(r, Err(e) if e.kind() == std::io::ErrorKind::NotFound))
                {
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
        let feedback = self.motors_supervisors
            .values()
            .flat_map(|supervisor| supervisor.get_latest_feedback())
            .collect::<HashMap<_, _>>();
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
