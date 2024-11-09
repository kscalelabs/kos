
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
