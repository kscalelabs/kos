#![allow(unknown_lints)]
#![allow(clippy::doc_lazy_continuation)]

pub mod config;
pub mod daemon;
pub mod file_logging;
mod grpc_interface;
pub mod hal;
pub mod services;
pub mod telemetry;
pub mod telemetry_types;

pub use grpc_interface::google as google_proto;
pub use grpc_interface::kos as kos_proto;

use async_trait::async_trait;
use hal::actuator_service_server::ActuatorServiceServer;
use hal::imu_service_server::ImuServiceServer;
use hal::inference_service_server::InferenceServiceServer;
use hal::process_manager_service_server::ProcessManagerServiceServer;
use services::OperationsServiceImpl;
use services::{
    ActuatorServiceImpl, IMUServiceImpl, InferenceServiceImpl, ProcessManagerServiceImpl,
};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

impl Debug for ActuatorServiceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActuatorServiceImpl")
    }
}

impl Debug for IMUServiceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMUServiceImpl")
    }
}

impl Debug for ProcessManagerServiceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcessManagerServiceImpl")
    }
}

impl Debug for InferenceServiceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InferenceServiceImpl")
    }
}

#[derive(Debug)]
pub enum ServiceEnum {
    Actuator(ActuatorServiceServer<ActuatorServiceImpl>),
    Imu(ImuServiceServer<IMUServiceImpl>),
    ProcessManager(ProcessManagerServiceServer<ProcessManagerServiceImpl>),
    Inference(InferenceServiceServer<InferenceServiceImpl>),
}

#[async_trait]
pub trait Platform: Send + Sync {
    fn name(&self) -> &'static str;
    fn serial(&self) -> String;
    fn initialize(&mut self, operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()>;
    fn create_services<'a>(
        &'a self,
        operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>>;
    fn shutdown(&mut self) -> eyre::Result<()>;
}
