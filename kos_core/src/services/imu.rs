use crate::grpc_interface::google::longrunning::Operation;
use crate::hal::IMU;
use crate::kos_proto::common::ActionResponse;
use crate::kos_proto::imu::imu_service_server::ImuService;
use crate::kos_proto::imu::*;
use crate::telemetry::Telemetry;
use crate::telemetry_types::{EulerAngles, ImuValues, Quaternion};
use eyre::OptionExt;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct IMUServiceImpl {
    imu: Arc<dyn IMU>,
}

impl IMUServiceImpl {
    pub fn new(imu: Arc<dyn IMU>) -> Self {
        Self { imu }
    }
}

#[tonic::async_trait]
impl ImuService for IMUServiceImpl {
    async fn get_values(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ImuValuesResponse>, Status> {
        let values = self
            .imu
            .get_values()
            .await
            .map_err(|e| Status::internal(format!("Failed to get IMU values, {:?}", e)))?;

        let telemetry = Telemetry::get().await;
        if let Some(telemetry) = telemetry {
            if let Err(e) = telemetry
                .publish("imu/values", &ImuValues::from(&values))
                .await
            {
                tracing::warn!("Failed to publish telemetry: {}", e);
            }
        }

        trace!("Getting IMU values, response: {:?}", values);

        Ok(Response::new(values))
    }

    async fn calibrate(&self, _request: Request<()>) -> Result<Response<Operation>, Status> {
        let _status = self
            .imu
            .calibrate()
            .await
            .map_err(|e| Status::internal(format!("Failed to calibrate IMU, {:?}", e)))?;

        Ok(Response::new(Operation {
            name: "operations/calibrate_imu/0".to_string(),
            metadata: None,
            done: false,
            result: None,
        }))
    }

    async fn zero(
        &self,
        request: Request<ZeroImuRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        let duration = request
            .into_inner()
            .duration
            .ok_or_eyre("Duration is required")
            .map_err(|_| Status::internal("Failed to parse duration"))?;

        let duration = std::time::Duration::from_nanos(duration.nanos as u64)
            + std::time::Duration::from_secs(duration.seconds as u64);

        let response = self
            .imu
            .zero(duration)
            .await
            .map_err(|e| Status::internal(format!("Failed to zero IMU, {:?}", e)))?;
        Ok(Response::new(response))
    }

    async fn get_euler(
        &self,
        _request: Request<()>,
    ) -> Result<Response<EulerAnglesResponse>, Status> {
        let euler = self
            .imu
            .get_euler()
            .await
            .map_err(|e| Status::internal(format!("Failed to get euler, {:?}", e)))?;

        let telemetry = Telemetry::get().await;
        if let Some(telemetry) = telemetry {
            if let Err(e) = telemetry
                .publish("imu/euler", &EulerAngles::from(&euler))
                .await
            {
                tracing::warn!("Failed to publish telemetry: {}", e);
            }
        }

        Ok(Response::new(euler))
    }

    async fn get_quaternion(
        &self,
        _request: Request<()>,
    ) -> Result<Response<QuaternionResponse>, Status> {
        let quaternion = self
            .imu
            .get_quaternion()
            .await
            .map_err(|e| Status::internal(format!("Failed to get quaternion, {:?}", e)))?;

        let telemetry = Telemetry::get().await;
        if let Some(telemetry) = telemetry {
            if let Err(e) = telemetry
                .publish("imu/quaternion", &Quaternion::from(&quaternion))
                .await
            {
                tracing::warn!("Failed to publish telemetry: {}", e);
            }
        }

        Ok(Response::new(quaternion))
    }
}
