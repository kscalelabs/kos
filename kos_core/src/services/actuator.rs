use crate::grpc_interface::google::longrunning::Operation;
use crate::hal::Actuator;
use crate::kos_proto::actuator::actuator_service_server::ActuatorService;
use crate::kos_proto::actuator::*;
use crate::kos_proto::common::ActionResponse;
use crate::telemetry::Telemetry;
use crate::telemetry_types::{ActuatorCommand, ActuatorState};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct ActuatorServiceImpl {
    actuator: Arc<dyn Actuator>,
}

impl ActuatorServiceImpl {
    pub fn new(actuator: Arc<dyn Actuator>) -> Self {
        Self { actuator }
    }
}

#[tonic::async_trait]
impl ActuatorService for ActuatorServiceImpl {
    async fn command_actuators(
        &self,
        request: Request<CommandActuatorsRequest>,
    ) -> Result<Response<CommandActuatorsResponse>, Status> {
        let commands = request.into_inner().commands;

        let telemetry_commands: Vec<_> = commands.iter().map(ActuatorCommand::from).collect();

        let results = self
            .actuator
            .command_actuators(commands)
            .await
            .map_err(|e| Status::internal(format!("Failed to command actuators, {:?}", e)))?;

        trace!(
            "Commanding actuators, request: {:?}, results: {:?}",
            telemetry_commands.clone(),
            results
        );

        let telemetry = Telemetry::get().await;
        if let Some(telemetry) = telemetry {
            if let Err(e) = telemetry
                .publish("actuator/command", &telemetry_commands)
                .await
            {
                tracing::warn!("Failed to publish telemetry: {}", e);
            }
        }

        Ok(Response::new(CommandActuatorsResponse { results }))
    }

    async fn configure_actuator(
        &self,
        request: Request<ConfigureActuatorRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        let config = request.into_inner();
        let response = self
            .actuator
            .configure_actuator(config)
            .await
            .map_err(|e| Status::internal(format!("Failed to configure actuator, {:?}", e)))?;
        Ok(Response::new(response))
    }

    async fn calibrate_actuator(
        &self,
        request: Request<CalibrateActuatorRequest>,
    ) -> Result<Response<Operation>, Status> {
        let calibrate_request = request.into_inner();
        let operation = self
            .actuator
            .calibrate_actuator(calibrate_request)
            .await
            .map_err(|e| Status::internal(format!("Failed to calibrate actuator, {:?}", e)))?;

        Ok(Response::new(operation))
    }

    async fn get_actuators_state(
        &self,
        request: Request<GetActuatorsStateRequest>,
    ) -> Result<Response<GetActuatorsStateResponse>, Status> {
        let request = request.into_inner();
        let actuator_ids = request.actuator_ids.clone();
        let states = self
            .actuator
            .get_actuators_state(actuator_ids)
            .await
            .map_err(|e| Status::internal(format!("Failed to get actuators state, {:?}", e)))?;

        let telemetry_states: Vec<_> = states.iter().map(ActuatorState::from).collect();
        let telemetry = Telemetry::get().await;
        if let Some(telemetry) = telemetry {
            if let Err(e) = telemetry.publish("actuator/state", &telemetry_states).await {
                tracing::warn!("Failed to publish telemetry: {}", e);
            }
        }

        trace!(
            "Getting actuators state, request: {:?}, response: {:?}",
            request.clone(),
            states
        );
        Ok(Response::new(GetActuatorsStateResponse { states }))
    }
}
