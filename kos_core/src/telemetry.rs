// TODO: Implement telemetry.
// General idea - MQTT for the robot, where serial of the robot is a topic.
// Mosquitto is the broker which will pass messages to InfluxDB
// We log desired vs actual joint angles (torque/velocity/position if applicable),
// as well as IMU data.

use eyre::Result;
use lazy_static::lazy_static;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Telemetry {
    client: Arc<AsyncClient>,
    robot_id: String,
    frame_number: Arc<AtomicU64>,
    video_timestamp: Arc<AtomicU64>,
    inference_step: Arc<AtomicU64>,
}

lazy_static! {
    static ref TELEMETRY: Arc<Mutex<Option<Telemetry>>> = Arc::new(Mutex::new(None));
}

#[derive(Serialize)]
struct TelemetryPayload<T> {
    frame_number: u64,
    video_timestamp: u64,
    inference_step: u64,
    data: T,
}

impl Telemetry {
    pub async fn initialize(robot_id: &str, mqtt_host: &str, mqtt_port: u16) -> Result<()> {
        let mut mqtt_options = MqttOptions::new(format!("kos-{}", robot_id), mqtt_host, mqtt_port);
        mqtt_options.set_keep_alive(std::time::Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

        // Spawn a task to handle MQTT connection events
        tokio::spawn(async move {
            while let Ok(notification) = eventloop.poll().await {
                tracing::trace!("MQTT Event: {:?}", notification);
            }
        });

        let telemetry = Telemetry {
            client: Arc::new(client),
            robot_id: robot_id.to_string(),
            frame_number: Arc::new(AtomicU64::new(0)),
            video_timestamp: Arc::new(AtomicU64::new(0)),
            inference_step: Arc::new(AtomicU64::new(0)),
        };

        tracing::debug!("Initializing telemetry for robot {}", robot_id);
        let mut global = TELEMETRY.lock().await;
        *global = Some(telemetry);

        Ok(())
    }

    pub async fn get() -> Option<Telemetry> {
        TELEMETRY.lock().await.clone()
    }

    pub async fn publish<T: Serialize>(&self, topic: &str, payload: &T) -> Result<()> {
        let telemetry_payload = TelemetryPayload {
            frame_number: self.get_frame_number(),
            video_timestamp: self.get_video_timestamp(),
            inference_step: self.get_inference_step(),
            data: payload,
        };

        let payload = serde_json::to_string(&telemetry_payload)?;
        let full_topic = format!("robots/{}/{}", self.robot_id, topic);

        self.client
            .publish(full_topic, QoS::AtLeastOnce, false, payload)
            .await?;

        Ok(())
    }

    pub fn update_frame_number(&self, new_frame_number: u64) {
        self.frame_number.store(new_frame_number, Ordering::SeqCst);
    }

    pub fn update_video_timestamp(&self, new_video_timestamp: u64) {
        self.video_timestamp
            .store(new_video_timestamp, Ordering::SeqCst);
    }

    pub fn get_frame_number(&self) -> u64 {
        self.frame_number.load(Ordering::SeqCst)
    }

    pub fn increment_frame_number(&self) {
        self.frame_number.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_video_timestamp(&self) -> u64 {
        self.video_timestamp.load(Ordering::SeqCst)
    }

    pub fn update_inference_step(&self, new_inference_step: u64) {
        self.inference_step
            .store(new_inference_step, Ordering::SeqCst);
    }

    pub fn increment_inference_step(&self) {
        self.inference_step.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_inference_step(&self) -> u64 {
        self.inference_step.load(Ordering::SeqCst)
    }
}
