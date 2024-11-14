use async_trait::async_trait;
use eyre::Result;
use kos_core::google_proto::longrunning::Operation;
use kos_core::services::OperationsServiceImpl;
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorRequest},
    kos_proto::{actuator::*, common::ActionResult},
};
use std::collections::HashMap;
use std::sync::Arc;

use robstride::{MotorMode, MotorType, Motors, MotorsSupervisor};

pub struct KscaleProActuator {
    motors_supervisor: MotorsSupervisor,
}

impl KscaleProActuator {
    pub fn new(
        port: &str,
        motor_infos: HashMap<u32, MotorType>,
        verbose: Option<bool>,
        max_update_rate: Option<u32>,
        zero_on_init: Option<bool>,
    ) -> Self {

        let motors_supervisor = MotorsSupervisor::new(
            port,
            &motor_infos,
            verbose.unwrap_or(false),
            max_update_rate.unwrap_or(100000),
            zero_on_init.unwrap_or(false),
        )
        .unwrap();

        KscaleProActuator {
            motors_supervisor,
        }
    }
}

#[async_trait]
impl Actuator for KscaleProActuator {
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

    async fn calibrate_actuator(&self, request: CalibrateActuatorRequest) -> Result<Operation> {
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
                actuator_id: *id,
                online: state.mode==MotorMode.Motor,
                position: state.position,
                velocity: state.velocity,
                torque: state.torque,
                temperature: None,
                voltage: None,
                current: None,
            })
            .collect())
    }
}
