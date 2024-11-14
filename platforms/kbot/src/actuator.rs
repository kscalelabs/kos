use crate::{Arc, Operation, OperationsServiceImpl};
use async_trait::async_trait;
use eyre::{Result, WrapErr};
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorRequest},
    kos_proto::{actuator::*, common::ActionResult},
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
        _commands: Vec<ActuatorCommand>,
    ) -> Result<Vec<ActionResult>> {
        Ok(vec![])
    }

    async fn configure_actuator(
        &self,
        _config: ConfigureActuatorRequest,
    ) -> Result<ActionResponse> {
        Ok(ActionResponse {
            success: true,
            error: None,
        })
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
                position: Some(state.position as f64),
                velocity: Some(state.velocity as f64),
                torque: Some(state.torque as f64),
                temperature: None,
                voltage: None,
                current: None,
            })
            .collect())
    }
}
