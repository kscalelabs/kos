use crate::grpc_interface::kos::imu::{EulerAnglesResponse, ImuValuesResponse, QuaternionResponse};
use serde::Serialize;
use crate::grpc_interface::kos::actuator::{ActuatorStateResponse, ActuatorCommand as ProtoActuatorCommand};

#[derive(Serialize)]
pub struct ImuValues {
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
    pub gyro_x: f64,
    pub gyro_y: f64,
    pub gyro_z: f64,
    pub mag_x: Option<f64>,
    pub mag_y: Option<f64>,
    pub mag_z: Option<f64>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct EulerAngles {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

#[derive(Serialize)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Serialize)]
pub struct ActuatorState {
    pub actuator_id: u32,
    pub online: bool,
    pub position: Option<f64>,
    pub velocity: Option<f64>,
    pub torque: Option<f64>,
    pub temperature: Option<f64>,
    pub voltage: Option<f32>,
    pub current: Option<f32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ActuatorCommand {
    pub actuator_id: u32,
    pub position: Option<f64>,
    pub velocity: Option<f64>,
    pub torque: Option<f64>,
}

impl From<&EulerAnglesResponse> for EulerAngles {
    fn from(resp: &EulerAnglesResponse) -> Self {
        Self {
            roll: resp.roll,
            pitch: resp.pitch,
            yaw: resp.yaw,
        }
    }
}

impl From<&ImuValuesResponse> for ImuValues {
    fn from(resp: &ImuValuesResponse) -> Self {
        Self {
            accel_x: resp.accel_x,
            accel_y: resp.accel_y,
            accel_z: resp.accel_z,
            gyro_x: resp.gyro_x,
            gyro_y: resp.gyro_y,
            gyro_z: resp.gyro_z,
            mag_x: resp.mag_x,
            mag_y: resp.mag_y,
            mag_z: resp.mag_z,
            error: resp.error.as_ref().map(|e| e.message.clone()),
        }
    }
}

impl From<&QuaternionResponse> for Quaternion {
    fn from(resp: &QuaternionResponse) -> Self {
        Self {
            x: resp.x,
            y: resp.y,
            z: resp.z,
            w: resp.w,
        }
    }
}

impl From<&ActuatorStateResponse> for ActuatorState {
    fn from(resp: &ActuatorStateResponse) -> Self {
        Self {
            actuator_id: resp.actuator_id,
            online: resp.online,
            position: resp.position,
            velocity: resp.velocity,
            torque: resp.torque,
            temperature: resp.temperature,
            voltage: resp.voltage,
            current: resp.current,
        }
    }
}

impl From<&ProtoActuatorCommand> for ActuatorCommand {
    fn from(cmd: &ProtoActuatorCommand) -> Self {
        Self {
            actuator_id: cmd.actuator_id,
            position: cmd.position,
            velocity: cmd.velocity,
            torque: cmd.torque,
        }
    }
}
