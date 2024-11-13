use async_trait::async_trait;
use eyre::Result;
use kos_core::google_proto::longrunning::Operation;
use kos_core::services::OperationsServiceImpl;
use kos_core::{
    hal::{
        ActionResponse, Actuator, ActuatorCommand, CalibrateActuatorMetadata,
        CalibrateActuatorRequest, CalibrationStatus,
    },
    kos_proto::{actuator::*, common::ActionResult},
};
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
        }])
    }
}
