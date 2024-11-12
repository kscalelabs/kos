#![allow(unknown_lints)]
#![allow(clippy::doc_lazy_continuation)]

pub mod config;
mod grpc_interface;
pub mod hal;
pub mod process_manager;
pub mod services;
pub mod telemetry;

pub use grpc_interface::google as google_proto;
pub use grpc_interface::kos as kos_proto;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config_loading() {
        let yaml = r#"
        limbs:
            LeftArm:
                port_name: /dev/ttyUSB0
                motor_configs:
                    1:
                        motor_type: Type01
                        kp: 50.0
                        kd: 1.0
        "#;
        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        assert_eq!(config.limbs.len(), 1);
        assert_eq!(config.limbs.contains_key("LeftArm"), true);
    }
}
