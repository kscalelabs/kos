// TODO: Implement telemetry.
// General idea - MQTT for the robot, where serial of the robot is a topic.
// Mosquitto is the broker which will pass messages to InfluxDB
// We log desired vs actual joint angles (torque/velocity/position if applicable),
// as well as IMU data.

use eyre::Result;
use lazy_static::lazy_static;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Telemetry {
    client: Arc<AsyncClient>,
    robot_id: String,
}

lazy_static! {
    static ref TELEMETRY: Arc<Mutex<Option<Telemetry>>> = Arc::new(Mutex::new(None));
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
        let payload = serde_json::to_string(payload)?;
        let full_topic = format!("robots/{}/{}", self.robot_id, topic);

        self.client
            .publish(full_topic, QoS::AtLeastOnce, false, payload)
            .await?;

        Ok(())
    }
}
