pub use crate::grpc_interface::google::longrunning::*;
pub use crate::grpc_interface::kos;
pub use crate::grpc_interface::kos::common::ActionResponse;
pub use crate::kos_proto::{actuator::*, common::ActionResult, imu::*, process_manager::*};
use async_trait::async_trait;
use eyre::Result;
use std::fmt::Display;

#[async_trait]
pub trait Actuator: Send + Sync {
    async fn command_actuators(&self, commands: Vec<ActuatorCommand>) -> Result<Vec<ActionResult>>;
    async fn configure_actuator(&self, config: ConfigureActuatorRequest) -> Result<ActionResponse>;
    async fn calibrate_actuator(&self, request: CalibrateActuatorRequest) -> Result<Operation>;
    async fn get_actuators_state(
        &self,
        actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>>;
}

#[async_trait]
pub trait IMU: Send + Sync {
    async fn get_values(&self) -> Result<ImuValuesResponse>;
    async fn calibrate(&self) -> Result<Operation>;
    async fn zero(
        &self,
        duration: Option<std::time::Duration>,
        max_retries: Option<u32>,
        max_angular_error: Option<f32>,
        max_vel: Option<f32>,
        max_accel: Option<f32>,
    ) -> Result<ActionResponse>;
    async fn get_euler(&self) -> Result<EulerAnglesResponse>;
    async fn get_quaternion(&self) -> Result<QuaternionResponse>;
}

#[async_trait]
pub trait ProcessManager: Send + Sync {
    async fn start_kclip(&self, action: String) -> Result<KClipStartResponse>;
    async fn stop_kclip(&self) -> Result<KClipStopResponse>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationStatus {
    Calibrating,
    Calibrated,
    Timeout,
}

impl Display for CalibrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalibrationStatus::Calibrating => write!(f, "calibrating"),
            CalibrationStatus::Calibrated => write!(f, "calibrated"),
            CalibrationStatus::Timeout => write!(f, "timeout"),
        }
    }
}
