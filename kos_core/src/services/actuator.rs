use crate::grpc_interface::google::longrunning::Operation;
use crate::hal::Actuator;
use crate::kos_proto::actuator::actuator_service_server::ActuatorService;
use crate::kos_proto::actuator::*;
use crate::kos_proto::common::ActionResponse;
use std::sync::Arc;
use tonic::{Request, Response, Status};

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
        let results = self
            .actuator
            .command_actuators(commands)
            .await
            .map_err(|e| Status::internal(format!("Failed to command actuators, {:?}", e)))?;
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
        let _status = self
            .actuator
            .calibrate_actuator(calibrate_request)
            .await
            .map_err(|e| Status::internal(format!("Failed to calibrate actuator, {:?}", e)))?;

        Ok(Response::new(Operation {
            name: "operations/calibrate_actuator/0".to_string(),
            metadata: None,
            done: false,
            result: None,
        }))
    }

    async fn get_actuators_state(
        &self,
        request: Request<GetActuatorsStateRequest>,
    ) -> Result<Response<GetActuatorsStateResponse>, Status> {
        let actuator_ids = request.into_inner().actuator_ids;
        let states = self
            .actuator
            .get_actuators_state(actuator_ids)
            .await
            .map_err(|e| Status::internal(format!("Failed to get actuators state, {:?}", e)))?;
        Ok(Response::new(GetActuatorsStateResponse { states }))
    }
}
