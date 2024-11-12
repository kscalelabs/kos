pub use crate::grpc_interface::google::longrunning::Operation;
pub use crate::grpc_interface::kos;
pub use crate::grpc_interface::kos::common::ActionResponse;
pub use crate::kos_proto::{actuator::*, common::ActionResult, imu::*};
use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait Actuator: Send + Sync {
    async fn command_actuators(&self, commands: Vec<ActuatorCommand>) -> Result<Vec<ActionResult>>;
    async fn configure_actuator(&self, config: ConfigureActuatorRequest) -> Result<()>;
    async fn calibrate_actuator(&self, request: CalibrateActuatorRequest) -> Result<()>;
    async fn get_actuators_state(
        &self,
        actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>>;
}

#[async_trait]
pub trait IMU: Send + Sync {
    async fn get_values(&self) -> Result<ImuValuesResponse>;
    async fn calibrate(&self) -> Result<CalibrationStatus>;
    async fn zero(&self, duration: std::time::Duration) -> Result<ActionResponse>;
    async fn get_euler(&self) -> Result<EulerAnglesResponse>;
    async fn get_quaternion(&self) -> Result<QuaternionResponse>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationStatus {
    Calibrating,
    Calibrated,
    Timeout,
}
