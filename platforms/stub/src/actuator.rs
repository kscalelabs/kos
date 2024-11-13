use async_trait::async_trait;
use eyre::Result;
use kos_core::google_proto::longrunning::Operation;
use kos_core::{
    hal::{
        ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorMetadata,
        CalibrateActuatorRequest, CalibrationStatus,
    },
    kos_proto::{actuator::*, common::ActionResult},
};
use prost::Message;
use prost_types::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::debug;
pub struct StubActuator {
    operations_store: Arc<Mutex<HashMap<String, Operation>>>,
}

impl StubActuator {
    pub fn new(operations_store: Arc<Mutex<HashMap<String, Operation>>>) -> Self {
        StubActuator { operations_store }
    }

    pub fn start_supervisor_task(operations_store: Arc<Mutex<HashMap<String, Operation>>>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(15)).await;
                let mut store = operations_store.lock().await;
                debug!("Checking operations store for calibration status, {:?}", store);
                for operation in store.values_mut() {
                    if let Some(metadata) = &mut operation.metadata {
                        if metadata.type_url == "type.googleapis.com/kos.actuator.CalibrateActuatorMetadata" {
                            // Decode the existing metadata
                            let mut decoded_metadata = CalibrateActuatorMetadata::decode(&metadata.value[..]).unwrap();
                            
                            // Update the status to calibrated
                            decoded_metadata.status = CalibrationStatus::Calibrated.to_string();
                            
                            // Re-encode the updated metadata
                            let mut buf = Vec::new();
                            decoded_metadata.encode(&mut buf).unwrap();
                            
                            // Update the operation's metadata with the new encoded value
                            metadata.value = buf;
                            
                            // Mark the operation as done
                            operation.done = true;
                        }
                    }
                }
            }
        });
    }
}

#[async_trait]
impl Actuator for StubActuator {
    async fn command_actuators(
        &self,
        _commands: Vec<ActuatorCommand>,
    ) -> Result<Vec<ActionResult>> {
        Ok(vec![])
    }

    async fn configure_actuator(
        &self,
        _config: ConfigureActuatorRequest,
    ) -> Result<ActionResponse> {
        Ok(ActionResponse {
            success: true,
            error: None,
        })
    }

    async fn calibrate_actuator(&self, request: CalibrateActuatorRequest) -> Result<Operation> {
        let metadata = CalibrateActuatorMetadata {
            actuator_id: request.actuator_id,
            status: CalibrationStatus::Calibrating.to_string(),
        };

        let mut buf = Vec::new();
        metadata.encode(&mut buf).unwrap();

        let operation = Operation {
            name: format!("operations/calibrate_actuator/{:?}", request.actuator_id),
            metadata: Some(Any {
                type_url: "type.googleapis.com/kos.actuator.CalibrateActuatorMetadata".to_string(),
                value: buf,
            }),
            done: false,
            result: None,
        };

        self.operations_store
            .lock()
            .await
            .insert(operation.name.clone(), operation.clone());

        Ok(operation)
    }

    async fn get_actuators_state(
        &self,
        _actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>> {
        Ok(vec![])
    }
}
