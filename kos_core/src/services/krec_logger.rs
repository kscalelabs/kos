use crate::kos_proto::{
    actuator::ActuatorStateResponse,
    imu::{ImuValuesResponse, QuaternionResponse},
};
use eyre::Result;
use krec::{
    ActuatorCommand, ActuatorState, ImuQuaternion, ImuValues, KRec, KRecFrame, KRecHeader, Vec3,
};
use prost::Message;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct ActuatorCommandData {
    frame_number: u64,
    video_timestamp: u64,
    inference_step: u64,
    data: Vec<ActuatorCommandItem>,
}

#[derive(Deserialize, Debug)]
struct ActuatorCommandItem {
    actuator_id: u32,
    position: Option<f64>,
    velocity: Option<f64>,
    torque: Option<f64>,
}

pub struct TelemetryLogger {
    krec: Arc<Mutex<KRec>>,
    _mqtt_client: AsyncClient,
    current_inference_step: Arc<Mutex<u64>>,
    current_frame: Arc<Mutex<KRecFrame>>,
    output_path: String,
}

impl TelemetryLogger {
    pub async fn new(
        uuid: String,
        action: String,
        output_path: impl AsRef<Path>,
        robot_name: String,
        robot_serial: String,
    ) -> Result<Self> {
        // Setup MQTT client
        let mut mqtt_options = MqttOptions::new("kos-telemetry-logger", "localhost", 1883);
        mqtt_options.set_keep_alive(std::time::Duration::from_secs(5));
        let (mqtt_client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Create KRec instance with header
        let header = KRecHeader {
            uuid,
            task: action,
            robot_platform: robot_name.clone(),
            robot_serial: robot_serial.clone(),
            start_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos() as u64,
            end_timestamp: 0,
            actuator_configs: vec![],
        };
        let krec = Arc::new(Mutex::new(KRec::new(header)));

        let current_inference_step = Arc::new(Mutex::new(0));
        let current_frame = Arc::new(Mutex::new(KRecFrame::default()));

        mqtt_client
            .subscribe(
                format!("robots/{}-{}/imu/values", robot_name, robot_serial),
                QoS::AtLeastOnce,
            )
            .await?;
        mqtt_client
            .subscribe(
                format!("robots/{}-{}/actuator/state", robot_name, robot_serial),
                QoS::AtLeastOnce,
            )
            .await?;
        mqtt_client
            .subscribe(
                format!("robots/{}-{}/actuator/command", robot_name, robot_serial),
                QoS::AtLeastOnce,
            )
            .await?;

        let output_path = output_path
            .as_ref()
            .to_str()
            .ok_or_else(|| eyre::eyre!("Failed to convert output path to string"))?;

        let logger = Self {
            krec,
            _mqtt_client: mqtt_client,
            current_inference_step,
            current_frame,
            output_path: output_path.to_owned(),
        };

        // Start processing MQTT messages
        let krec_clone = logger.krec.clone();
        let current_step = logger.current_inference_step.clone();
        let current_frame = logger.current_frame.clone();
        let output_path = output_path.to_owned();

        tokio::spawn(async move {
            while let Ok(event) = eventloop.poll().await {
                if let Event::Incoming(Packet::Publish(publish)) = event {
                    let payload = &publish.payload;
                    let topic = publish.topic;
                    let mut frame = current_frame.lock().await;

                    if topic.contains("/imu/values") {
                        if let Ok(imu_values) = ImuValuesResponse::decode(payload.as_ref()) {
                            // Convert IMUValuesResponse to ImuValues
                            frame.imu_values = Some(ImuValues {
                                accel: Some(Vec3 {
                                    x: imu_values.accel_x,
                                    y: imu_values.accel_y,
                                    z: imu_values.accel_z,
                                }),
                                gyro: Some(Vec3 {
                                    x: imu_values.gyro_x,
                                    y: imu_values.gyro_y,
                                    z: imu_values.gyro_z,
                                }),
                                mag: if imu_values.mag_x.is_some() {
                                    Some(Vec3 {
                                        x: imu_values.mag_x.unwrap_or_default(),
                                        y: imu_values.mag_y.unwrap_or_default(),
                                        z: imu_values.mag_z.unwrap_or_default(),
                                    })
                                } else {
                                    None
                                },
                                quaternion: None,
                            });
                        } else {
                            tracing::error!("Failed to decode ImuValuesResponse {:?}", payload);
                        }
                    } else if topic.contains("/imu/quaternion") {
                        if let Ok(quat) = QuaternionResponse::decode(payload.as_ref()) {
                            // Update quaternion in the current IMU values
                            if frame.imu_values.is_none() {
                                frame.imu_values = Some(ImuValues::default());
                            }
                            if let Some(ref mut imu_values) = frame.imu_values {
                                imu_values.quaternion = Some(ImuQuaternion {
                                    x: quat.x,
                                    y: quat.y,
                                    z: quat.z,
                                    w: quat.w,
                                });
                            }
                        } else {
                            tracing::error!("Failed to decode QuaternionResponse {:?}", payload);
                        }
                    } else if topic.contains("/actuator/state") {
                        if let Ok(state) = ActuatorStateResponse::decode(payload.as_ref()) {
                            frame.actuator_states.push(ActuatorState {
                                actuator_id: state.actuator_id,
                                online: state.online,
                                position: state.position,
                                velocity: state.velocity,
                                torque: state.torque,
                                temperature: state.temperature,
                                voltage: state.voltage,
                                current: state.current,
                            });
                        } else {
                            tracing::error!("Failed to decode ActuatorStateResponse {:?}", payload);
                        }
                    } else if topic.contains("/actuator/command") {
                        match serde_json::from_slice::<ActuatorCommandData>(&payload) {
                            Ok(command_data) => {
                                frame.inference_step = command_data.inference_step;
                                frame.video_timestamp = command_data.video_timestamp;
                                frame.frame_number = command_data.frame_number;

                                for item in command_data.data {
                                    frame.actuator_commands.push(ActuatorCommand {
                                        actuator_id: item.actuator_id,
                                        position: item.position.unwrap_or_default() as f32,
                                        velocity: item.velocity.unwrap_or_default() as f32,
                                        torque: item.torque.unwrap_or_default() as f32,
                                    });
                                }
                                tracing::debug!("Parsed actuator command: {:?}", frame);
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse actuator command JSON: {:?}", e);
                            }
                        }
                    }

                    // Check if inference step has increased
                    let mut current = current_step.lock().await;
                    if frame.inference_step > *current {
                        // Add frame to KRec
                        let mut krec = krec_clone.lock().await;

                        krec.add_frame(frame.clone());

                        // Save every 500 frames
                        if krec.frames.len() % 500 == 0 {
                            if let Err(e) = krec.save(&output_path) {
                                tracing::warn!("Failed to save KRec file: {}", e);
                            } else {
                                tracing::debug!("Saved {} frames to KRec file", krec.frames.len());
                            }
                        }
                        // Reset frame for next step
                        *frame = KRecFrame::default();
                        *current = frame.inference_step;
                    }
                }
            }
        });

        Ok(logger)
    }

    pub async fn stop(&self) -> Result<()> {
        let mut krec = self.krec.lock().await;

        // Update end timestamp
        krec.header.end_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos() as u64;

        // Save final state
        krec.save(&self.output_path)?;
        tracing::info!("Saved final KRec file with {} frames", krec.frames.len());

        Ok(())
    }
}
