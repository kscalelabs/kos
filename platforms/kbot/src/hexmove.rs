use kos_core::{
    hal::{EulerAnglesResponse, ImuAdvancedValuesResponse, ImuValuesResponse, Operation, QuaternionResponse, IMU},
    kos_proto::common::{ActionResponse, Error, ErrorCode},
    services::OperationsServiceImpl,
};

use async_trait::async_trait;
use eyre::{Result, WrapErr};
use imu::hexmove::*;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, trace};

pub struct KBotIMU {
    _operations_service: Arc<OperationsServiceImpl>,
    imu: ImuReader,
}

impl KBotIMU {
    pub fn new(
        operations_service: Arc<OperationsServiceImpl>,
        interface: &str,
        can_id: u32,
        model: u32,
    ) -> Result<Self> {
        info!(
            "Initializing KBotIMU with interface: {}, CAN ID: {}, model: {}",
            interface, can_id, model
        );

        let can_id =
            u8::try_from(can_id).wrap_err_with(|| format!("CAN ID {} too large for u8", can_id))?;
        let model =
            u8::try_from(model).wrap_err_with(|| format!("Model ID {} too large for u8", model))?;

        let imu = match ImuReader::new(interface, can_id, model) {
            Ok(imu) => {
                info!("Successfully created IMU reader");
                imu
            }
            Err(e) => {
                error!("Failed to create IMU reader: {}", e);
                return Err(eyre::eyre!("Failed to create IMU reader: {}", e));
            }
        };

        Ok(KBotIMU {
            _operations_service: operations_service,
            imu,
        })
    }
}

impl Default for KBotIMU {
    fn default() -> Self {
        unimplemented!("KBotIMU cannot be default, it requires an operations store")
    }
}

#[async_trait]
impl IMU for KBotIMU {
    async fn get_values(&self) -> Result<ImuValuesResponse> {
        let data = self
            .imu
            .get_data()
            .map_err(|e| eyre::eyre!("Failed to get IMU data: {}", e))?;
        trace!(
            "Reading IMU values, accel x: {}, y: {}, z: {}, angle x: {}, y: {}, z: {}",
            data.x_velocity,
            data.y_velocity,
            data.z_velocity,
            data.x_angle,
            data.y_angle,
            data.z_angle
        );

        Ok(ImuValuesResponse {
            accel_x: data.x_velocity as f64,
            accel_y: data.y_velocity as f64,
            accel_z: data.z_velocity as f64,
            gyro_x: 0 as f64,
            gyro_y: 0 as f64,
            gyro_z: 0 as f64,
            mag_x: None,
            mag_y: None,
            mag_z: None,
            grav_x: None,
            grav_y: None,
            grav_z: None,
            error: None,
        })
    }

    async fn get_advanced_values(&self) -> Result<ImuAdvancedValuesResponse> {
        Ok(ImuAdvancedValuesResponse {
            lin_acc_x: None,
            lin_acc_y: None,
            lin_acc_z: None,
            grav_x: None,
            grav_y: None,
            grav_z: None,
            temp: None,
            error: None,
        })
    }

    async fn calibrate(&self) -> Result<Operation> {
        info!("Starting IMU calibration - unimplemented");
        Ok(Operation {
            name: "operations/calibrate_imu/0".to_string(),
            metadata: None,
            done: true,
            result: None,
        })
    }

    async fn zero(
        &self,
        duration: Option<Duration>,
        max_retries: Option<u32>,
        max_angular_error: Option<f32>,
        _max_vel: Option<f32>,
        _max_accel: Option<f32>,
    ) -> Result<ActionResponse> {
        if self
            .imu
            .zero_imu(
                duration.map(|d| d.as_millis() as u64),
                max_retries,
                max_angular_error,
            )
            .is_err()
        {
            return Ok(ActionResponse {
                success: false,
                error: Some(Error {
                    code: ErrorCode::HardwareFailure as i32,
                    message: "Failed to zero IMU".to_string(),
                }),
            });
        }
        Ok(ActionResponse {
            success: true,
            error: None,
        })
    }

    async fn get_euler(&self) -> Result<EulerAnglesResponse> {
        debug!("Reading Euler angles");
        let data = self
            .imu
            .get_data()
            .map_err(|e| eyre::eyre!("Failed to get IMU data: {}", e))?;
        Ok(EulerAnglesResponse {
            roll: data.x_angle as f64,
            pitch: data.y_angle as f64,
            yaw: data.z_angle as f64,
            error: None,
        })
    }

    async fn get_quaternion(&self) -> Result<QuaternionResponse> {
        debug!("Reading quaternion");
        let data = self
            .imu
            .get_data()
            .map_err(|e| eyre::eyre!("Failed to get IMU data: {}", e))?;
        Ok(QuaternionResponse {
            w: data.qw as f64,
            x: data.qx as f64,
            y: data.qy as f64,
            z: data.qz as f64,
            error: None,
        })
    }
}
