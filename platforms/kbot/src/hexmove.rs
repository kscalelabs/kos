use kos_core::{
    hal::{EulerAnglesResponse, ImuValuesResponse, Operation, QuaternionResponse, IMU},
    kos_proto::common::ActionResponse,
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
        let data = self.imu.get_data();
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

    async fn zero(&self, duration: Duration) -> Result<ActionResponse> {
        info!("Zeroing IMU with duration: {:?} - unimplemented", duration);
        Ok(ActionResponse {
            success: true,
            error: None,
        })
    }

    async fn get_euler(&self) -> Result<EulerAnglesResponse> {
        debug!("Reading Euler angles");
        let data = self.imu.get_data();
        Ok(EulerAnglesResponse {
            roll: data.x_angle as f64,
            pitch: data.y_angle as f64,
            yaw: data.z_angle as f64,
            error: None,
        })
    }

    async fn get_quaternion(&self) -> Result<QuaternionResponse> {
        error!("Quaternion operation not implemented");
        Err(eyre::eyre!("Not implemented"))
    }
}
