use async_trait::async_trait;
use eyre::Result;
use kos_core::{
    google_proto::longrunning::Operation,
    hal::{EulerAnglesResponse, ImuValuesResponse, QuaternionResponse, IMU},
    kos_proto::common::ActionResponse,
};
use std::time::Duration;

pub struct SimIMU {}

impl SimIMU {
    pub fn new() -> Self {
        SimIMU {}
    }
}

impl Default for SimIMU {
    fn default() -> Self {
        SimIMU::new()
    }
}

#[async_trait]
impl IMU for SimIMU {
    async fn get_values(&self) -> Result<ImuValuesResponse> {
        todo!()
    }

    async fn calibrate(&self) -> Result<Operation> {
        todo!()
    }

    async fn zero(&self, _duration: Duration) -> Result<ActionResponse> {
        todo!()
    }

    async fn get_euler(&self) -> Result<EulerAnglesResponse> {
        todo!()
    }

    async fn get_quaternion(&self) -> Result<QuaternionResponse> {
        todo!()
    }
}
