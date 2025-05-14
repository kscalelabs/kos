mod actuator;
mod imu;
mod policy;
mod process_manager;
use crate::actuator::StubActuator;
use crate::imu::StubIMU;
use crate::policy::StubPolicy;
use crate::process_manager::StubProcessManager;
use async_trait::async_trait;
use kos::hal::Operation;
use kos::kos_proto::actuator::actuator_service_server::ActuatorServiceServer;
use kos::kos_proto::imu::imu_service_server::ImuServiceServer;
use kos::kos_proto::policy::policy_service_server::PolicyServiceServer;
use kos::kos_proto::process_manager::process_manager_service_server::ProcessManagerServiceServer;
use kos::services::{
    ActuatorServiceImpl, IMUServiceImpl, PolicyServiceImpl, ProcessManagerServiceImpl,
};
use kos::{services::OperationsServiceImpl, Platform, ServiceEnum};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct StubPlatform {}

impl StubPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StubPlatform {
    fn default() -> Self {
        StubPlatform::new()
    }
}

#[async_trait]
impl Platform for StubPlatform {
    fn name(&self) -> &'static str {
        "Stub"
    }

    fn serial(&self) -> String {
        "00000000".to_string()
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        // Initialize the platform
        Ok(())
    }

    fn create_services<'a>(
        &'a self,
        operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>> {
        Box::pin(async move {
            let actuator = StubActuator::new(operations_service.clone());
            let imu = StubIMU::new(operations_service.clone());
            let process_manager = StubProcessManager::new();
            let policy = StubPolicy::new();

            Ok(vec![
                ServiceEnum::Actuator(ActuatorServiceServer::new(ActuatorServiceImpl::new(
                    Arc::new(actuator),
                ))),
                ServiceEnum::ProcessManager(ProcessManagerServiceServer::new(
                    ProcessManagerServiceImpl::new(Arc::new(process_manager)),
                )),
                ServiceEnum::Imu(ImuServiceServer::new(IMUServiceImpl::new(Arc::new(imu)))),
                ServiceEnum::Policy(PolicyServiceServer::new(
                    // Add this block
                    PolicyServiceImpl::new(Arc::new(policy)),
                )),
            ])
        })
    }

    fn shutdown(&mut self) -> eyre::Result<()> {
        // Shutdown and cleanup code goes here
        Ok(())
    }
}
