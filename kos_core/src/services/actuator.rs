use crate::hal::Actuator;
use crate::kos_proto::actuator::*;
use tonic::{Request, Response, Status};

pub struct ActuatorService {
    actuator: Box<dyn Actuator>,
}

// #[tonic::async_trait]
// impl ActuatorService {
//     async fn command_actuators(
//         &self,
//         request: Request<CommandActuatorsRequest>,
//     ) -> Result<Response<CommandActuatorsResponse>, Status> {
//         todo!()
//     }
// }
