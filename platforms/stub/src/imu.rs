use crate::Operation;
use async_trait::async_trait;
use eyre::Result;
use kos_core::services::OperationsServiceImpl;
use kos_core::{
    hal::{
        CalibrateImuMetadata, CalibrationStatus, EulerAnglesResponse, ImuAdvancedValuesResponse,
        ImuValuesResponse, QuaternionResponse, IMU,
    },
    kos_proto::common::ActionResponse,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct StubIMU {
    operations_service: Arc<OperationsServiceImpl>,
}

impl StubIMU {
    pub fn new(operations_service: Arc<OperationsServiceImpl>) -> Self {
        StubIMU { operations_service }
    }
}

impl Default for StubIMU {
    fn default() -> Self {
        unimplemented!("StubIMU cannot be default, it requires an operations store")
    }
}

#[async_trait]
impl IMU for StubIMU {
    async fn get_values(&self) -> Result<ImuValuesResponse> {
        Ok(ImuValuesResponse {
            accel_x: 1.0,
            accel_y: 2.0,
            accel_z: 3.0,
            gyro_x: 0.0,
            gyro_y: 0.0,
            gyro_z: 0.0,
            mag_x: None,
            mag_y: None,
            mag_z: None,
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
        let operation = Operation {
            name: format!("operations/imu/calibrate/{}", Uuid::new_v4()),
            metadata: None,
            done: false,
            result: None,
        };
        let metadata = CalibrateImuMetadata {
            status: CalibrationStatus::Calibrating.to_string(),
        };
        let operation = self
            .operations_service
            .create(
                operation.name,
                metadata,
                "type.googleapis.com/kos.imu.CalibrateIMUMetadata",
            )
            .await?;

        Ok(operation)
    }

    async fn zero(
        &self,
        _duration: Option<std::time::Duration>,
        _max_retries: Option<u32>,
        _max_angular_error: Option<f32>,
        _max_vel: Option<f32>,
        _max_accel: Option<f32>,
    ) -> Result<ActionResponse> {
        Ok(ActionResponse {
            success: true,
            error: None,
        })
    }

    async fn get_euler(&self) -> Result<EulerAnglesResponse> {
        Ok(EulerAnglesResponse {
            roll: 0.0,
            pitch: 30.0,
            yaw: 0.0,
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
