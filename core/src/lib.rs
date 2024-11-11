pub mod kos {
    pub mod actuator {
        tonic::include_proto!("kos/kos.actuator");
    }

    pub mod common {
        tonic::include_proto!("kos/kos.common");
    }

    pub mod imu {
        tonic::include_proto!("kos/kos.imu");
    }

    pub mod inference {
        tonic::include_proto!("kos/kos.inference");
    }

    pub mod process_manager {
        tonic::include_proto!("kos/kos.processmanager");
    }

    pub mod system {
        tonic::include_proto!("kos/kos.system");
    }
}

pub mod google {
    pub mod longrunning {
        tonic::include_proto!("kos/google.longrunning");
    }

    pub mod api {
        tonic::include_proto!("kos/google.api");
    }

    pub mod rpc {
        tonic::include_proto!("kos/google.rpc");
    }
}

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
