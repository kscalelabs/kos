use crate::Operation;
use async_trait::async_trait;
use eyre::Result;
use kos_core::{
    hal::{EulerAnglesResponse, ImuValuesResponse, QuaternionResponse, IMU},
    kos_proto::common::ActionResponse,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::collections::HashMap;

pub struct StubIMU {
    operations_store: Arc<Mutex<HashMap<String, Operation>>>,
}

impl StubIMU {
    pub fn new(operations_store: Arc<Mutex<HashMap<String, Operation>>>) -> Self {
        StubIMU { operations_store }
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

    async fn calibrate(&self) -> Result<Operation> {
        let operation = Operation {
            name: format!("operations/imu/calibrate/{}", Uuid::new_v4()),
            metadata: None,
            done: false,
            result: None,
        };
        {
            let mut store = self.operations_store.lock().await;
            store.insert(operation.name.clone(), operation.clone());
        }
        Ok(operation)
    }

    async fn zero(&self, _duration: Duration) -> Result<ActionResponse> {
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
