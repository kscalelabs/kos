use crate::kos_proto::{
    actuator::ActuatorStateResponse,
    imu::ImuValuesResponse,
    kclip::{KClipFrame, KClipHeader},
};
use base64::engine::general_purpose;
use base64::Engine;
use bytes::Bytes;
use eyre::Result;
use prost::Message;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TelemetryLogger {
    file: Arc<Mutex<File>>,
    _mqtt_client: AsyncClient,
    current_inference_step: Arc<Mutex<u64>>,
    current_frame: Arc<Mutex<KClipFrame>>,
}

impl TelemetryLogger {
    pub async fn new(
        uuid: String,
        action: String,
        output_path: impl AsRef<Path>,
        robot_name: String,
    ) -> Result<Self> {
        // Setup MQTT client
        let mut mqtt_options = MqttOptions::new("kos-telemetry-logger", "localhost", 1883);
        mqtt_options.set_keep_alive(std::time::Duration::from_secs(5));
        let (mqtt_client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Create file and write header
        let mut file = File::create(&output_path)?;
        let header = KClipHeader {
            uuid,
            action,
            start_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos() as u64,
            end_timestamp: 0,
            actuator_mappings: vec![],
        };

        // Write header as base64 encoded protobuf
        let mut header_buf = Vec::new();
        header.encode(&mut header_buf)?;
        writeln!(file, "{}", general_purpose::STANDARD.encode(&header_buf))?;

        let file = Arc::new(Mutex::new(file));
        let current_inference_step = Arc::new(Mutex::new(0));
        let current_frame = Arc::new(Mutex::new(KClipFrame::default()));

        // Subscribe to relevant topics
        mqtt_client
            .subscribe(
                format!("robots/{}/imu/values", robot_name),
                QoS::AtLeastOnce,
            )
            .await?;
        mqtt_client
            .subscribe(
                format!("robots/{}/actuator/state", robot_name),
                QoS::AtLeastOnce,
            )
            .await?;
        mqtt_client
            .subscribe(
                format!("robots/{}/actuator/command", robot_name),
                QoS::AtLeastOnce,
            )
            .await?;

        let logger = Self {
            file,
            _mqtt_client: mqtt_client,
            current_inference_step,
            current_frame,
        };

        // Start processing MQTT messages
        let file_clone = logger.file.clone();
        let current_step = logger.current_inference_step.clone();
        let current_frame = logger.current_frame.clone();

        tokio::spawn(async move {
            while let Ok(event) = eventloop.poll().await {
                if let Event::Incoming(Packet::Publish(publish)) = event {
                    let payload = &publish.payload;

                    // Try to decode the protobuf message based on the topic
                    let topic = publish.topic;
                    let mut frame = current_frame.lock().await;

                    if topic.contains("/imu/values") {
                        if let Ok(imu_values) = ImuValuesResponse::decode(payload.as_ref()) {
                            frame.imu_values = Some(imu_values);
                        }
                    } else if topic.contains("/actuator/state") {
                        if let Ok(state) = ActuatorStateResponse::decode(payload.as_ref()) {
                            frame.actuator_states.push(state);
                        }
                    }

                    // Check if inference step has increased
                    let mut current = current_step.lock().await;
                    if frame.inference_step > *current {
                        // Write the frame
                        if let Ok(mut file) = file_clone.try_lock() {
                            let mut frame_buf = Vec::new();
                            if frame.encode(&mut frame_buf).is_ok() {
                                writeln!(
                                    file,
                                    "{},{}",
                                    frame.frame_number,
                                    general_purpose::STANDARD.encode(&frame_buf)
                                )
                                .ok();
                            }
                        }
                        // Reset frame for next step
                        *frame = KClipFrame::default();
                        *current = frame.inference_step;
                    }
                }
            }
        });

        Ok(logger)
    }

    pub async fn stop(&self) -> Result<()> {
        let mut file = self.file.lock().await;

        // Read the current content
        let mut content = String::new();
        std::io::Read::read_to_string(&mut *file, &mut content)?;

        // Update header
        if let Some(header_b64) = content.lines().next() {
            let header_bytes = general_purpose::STANDARD.decode(header_b64)?;
            if let Ok(mut header) = KClipHeader::decode(Bytes::from(header_bytes)) {
                header.end_timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos() as u64;

                // Rewrite file with updated header
                file.set_len(0)?;
                file.seek(std::io::SeekFrom::Start(0))?;

                let mut header_buf = Vec::new();
                header.encode(&mut header_buf)?;
                writeln!(file, "{}", general_purpose::STANDARD.encode(&header_buf))?;
                write!(
                    file,
                    "{}",
                    content.lines().skip(1).collect::<Vec<_>>().join("\n")
                )?;
            }
        }

        Ok(())
    }
}
