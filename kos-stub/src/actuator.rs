use async_trait::async_trait;
use eyre::Result;
use kos::google_proto::longrunning::Operation;
use kos::services::OperationsServiceImpl;
use kos::{
    hal::{
        ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorMetadata,
        CalibrateActuatorRequest, CalibrationStatus,
    },
    kos_proto::{actuator::*, common::ActionResult},
};
use prost_types::{Struct, Value};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tokio::time::Duration;
use tracing::debug;
pub struct StubActuator {
    operations: Arc<OperationsServiceImpl>,
    calibration_tx: Sender<u32>,
}

impl StubActuator {
    pub fn new(operations: Arc<OperationsServiceImpl>) -> Self {
        let (tx, rx) = channel::<u32>();

        // Spawn the calibration thread
        let operations_clone = operations.clone();
        thread::spawn(move || {
            // Create a new runtime for this thread
            let rt = Runtime::new().expect("Failed to create runtime");

            loop {
                // Wait for actuator IDs to calibrate
                if let Ok(actuator_id) = rx.recv() {
                    let ops = operations_clone.clone();
                    debug!("Calibrating actuator ID: {}", actuator_id);

                    // Sleep for 15 seconds to simulate calibration
                    thread::sleep(Duration::from_secs(15));
                    debug!("Calibrated actuator ID: {}", actuator_id);

                    // Update the operation status
                    let operation_name = format!("operations/calibrate_actuator/{:?}", actuator_id);
                    debug!("Updating operation status for: {}", operation_name);

                    let metadata = CalibrateActuatorMetadata {
                        actuator_id,
                        status: CalibrationStatus::Calibrated.to_string(),
                    };

                    if let Err(e) =
                        rt.block_on(ops.update_metadata(&operation_name, metadata, true))
                    {
                        debug!("Failed to update calibration status: {}", e);
                    }

                    debug!("Updated operation status for: {}", operation_name);
                }
            }
        });

        StubActuator {
            operations,
            calibration_tx: tx,
        }
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

        let name = format!("operations/calibrate_actuator/{:?}", request.actuator_id);
        let operation = self
            .operations
            .create(
                name,
                metadata,
                "type.googleapis.com/kos.actuator.CalibrateActuatorMetadata",
            )
            .await
            .map_err(|e| eyre::eyre!("Failed to create operation: {}", e))?;

        // Send actuator ID to calibration thread
        self.calibration_tx
            .send(request.actuator_id)
            .map_err(|e| eyre::eyre!("Failed to start calibration: {}", e))?;

        Ok(operation)
    }

    async fn get_actuators_state(
        &self,
        _actuator_ids: Vec<u32>,
    ) -> Result<Vec<ActuatorStateResponse>> {
        Ok(vec![ActuatorStateResponse {
            actuator_id: 1,
            online: true,
            position: Some(0.0),
            velocity: Some(0.0),
            torque: Some(0.0),
            temperature: Some(0.0),
            voltage: Some(0.0),
            current: Some(0.0),
            faults: vec![],
            torque_enabled: Some(true),
            min_position: Some(-180.0),
            max_position: Some(180.0),
            kp: Some(1.0),
            kd: Some(0.1),
            ki: Some(0.01),
            max_torque: Some(5.0),
        }])
    }

    async fn get_parameters(&self, actuator_ids: Vec<u32>) -> Result<Vec<(u32, Struct)>> {
        let dummy_struct = Struct {
            fields: vec![
                ("model".to_string(), Value::from("stub")),
                ("firmware_version".to_string(), Value::from("0.0.1")),
                ("max_torque".to_string(), Value::from(0.5)),
            ]
            .into_iter()
            .collect(),
        };

        let results = actuator_ids
            .into_iter()
            .map(|id| (id, dummy_struct.clone()))
            .collect();

        Ok(results)
    }
}
