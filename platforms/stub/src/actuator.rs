use async_trait::async_trait;
use eyre::Result;
use kos_core::{
    hal::{ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorResponse},
    kos_proto::{actuator::*, common::ActionResult},
};

pub struct StubActuator {}

impl StubActuator {
    pub fn new() -> Self {
        StubActuator {}
    }
}

impl Default for StubActuator {
    fn default() -> Self {
        StubActuator::new()
    }
}

#[async_trait]
impl Actuator for StubActuator {
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

    async fn calibrate_actuator(
        &self,
        request: CalibrateActuatorRequest,
    ) -> Result<CalibrateActuatorResponse> {
        Ok(CalibrateActuatorResponse {
            actuator_id: request.actuator_id,
            error: None,
        })
    }

    async fn get_actuators_state(
        &self,
        _actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>> {
        Ok(vec![])
    }
}
