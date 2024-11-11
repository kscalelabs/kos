use async_trait::async_trait;
use eyre::Result;
use std::collections::HashMap;

#[async_trait]
pub trait Actuator: Send + Sync {
    async fn command_actuators(&self, commands: Vec<ActuatorCommand>) -> Result<Vec<ActionResult>>;
    async fn configure_actuator(&self, config: ConfigureActuatorRequest) -> Result<()>;
    async fn calibrate_actuator(&self, request: CalibrateActuatorRequest) -> Result<()>;
    async fn get_actuators_state(&self, actuator_ids: Vec<u32>) -> Result<Vec<ActuatorStateResponse>>;
}

#[async_trait]
pub trait IMU: Send + Sync {
    async fn get_values(&self) -> Result<IMUValuesResponse>;
    async fn calibrate(&self) -> Result<()>;
    async fn zero(&self, duration: std::time::Duration) -> Result<()>;
    async fn get_euler(&self) -> Result<EulerAnglesResponse>;
    async fn get_quaternion(&self) -> Result<QuaternionResponse>;
}

// Define data structures used by the traits
// These should mirror the data structures in your Protobuf definitions

#[derive(Debug)]
pub struct ActuatorCommand {
    pub actuator_id: u32,
    pub position: Option<f64>,
    pub velocity: Option<f64>,
    pub torque: Option<f64>,
}

#[derive(Debug)]
pub struct ConfigureActuatorRequest {
    pub actuator_id: u32,
    pub kp: f64,
    pub kd: f64,
    pub ki: f64,
    pub max_torque: f64,
    pub protective_torque: f64,
    pub protection_time: f32,
    pub torque_enabled: bool,
}

#[derive(Debug)]
pub struct CalibrateActuatorRequest {
    pub actuator_id: u32,
    pub calibration_speed: f64,
    pub threshold_current: f32,
}

#[derive(Debug)]
pub struct ActuatorStateResponse {
    pub actuator_id: u32,
    pub online: bool,
    pub position: f64,
    pub velocity: f64,
    pub torque: f64,
    pub temperature: f64,
    pub voltage: f32,
    pub current: f32,
}

#[derive(Debug)]
pub struct ActionResult {
    pub actuator_id: u32,
    pub success: bool,
    pub error: Option<HalError>,
}

#[derive(Debug)]
pub enum HalError {
    Unknown(String),
    NotImplemented(String),
    InvalidArgument(String),
    HardwareFailure(String),
    Timeout(String),
    Unauthorized(String),
}

#[derive(Debug)]
pub struct IMUValuesResponse {
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
    pub gyro_x: f64,
    pub gyro_y: f64,
    pub gyro_z: f64,
    pub mag_x: f64,
    pub mag_y: f64,
    pub mag_z: f64,
}

#[derive(Debug)]
pub struct EulerAnglesResponse {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

#[derive(Debug)]
pub struct QuaternionResponse {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
