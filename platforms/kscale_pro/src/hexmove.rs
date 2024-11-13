
use kos_core::{
    hal::{
        CalibrateImuMetadata, CalibrationStatus, EulerAnglesResponse, ImuValuesResponse,
        QuaternionResponse, IMU, Operation,
    },
    kos_proto::common::ActionResponse,
};

use imu::hexmove::*;

pub struct KscaleProIMU {
    operations_service: Arc<OperationsServiceImpl>,
    imu: ImuReader,
}

impl KscaleProIMU {
    pub fn new(interface: &str, can_id: u32, model: u32) -> Self {
        KscaleProIMU {
            operations_service,
            imu: ImuReader::new(interface, can_id, model).unwrap(),
        }
    }
}

impl Default for KscaleProIMU {
    fn default() -> Self {
        unimplemented!("KscaleProIMU cannot be default, it requires an operations store")
    }
}

#[async_trait]
impl IMU for StubIMU {
    async fn get_values(&self) -> Result<ImuValuesResponse> {
        let data = self.imu.get_data();
        Ok(ImuValuesResponse {
            accel_x: None,
            accel_y: None,
            accel_z: None,
            gyro_x: None,
            gyro_y: None,
            gyro_z: None,
            mag_x: None,
            mag_y: None,
            mag_z: None,
            error: None,
        })
    }

    async fn calibrate(&self) -> Result<Operation> {
        Ok(Operation::default())
    }

    async fn zero(&self, _duration: Duration) -> Result<ActionResponse> {
        Ok(ActionResponse {
            success: true,
            error: None,
        })
    }

    async fn get_euler(&self) -> Result<EulerAnglesResponse> {
        let data = self.imu.get_data();
        Ok(EulerAnglesResponse {
            roll: data.x_angle,
            pitch: data.y_angle,
            yaw: data.z_angle,
            error: None,
        })
    }

    async fn get_quaternion(&self) -> Result<QuaternionResponse> {
        Ok(QuaternionResponse {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            error: None,
        })
    }
}
