use robstride::MotorType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct MotorConfig {
    pub motor_type: MotorType,
    pub kp: f32,
    pub kd: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Limb {
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimbConfig {
    pub motor_configs: HashMap<u8, MotorConfig>,
    pub port_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub limbs: HashMap<Limb, LimbConfig>,
}

impl Config {
    pub fn new(config_path: &str) -> Self {
        let file = fs::File::open(config_path).expect("Failed to open config file");
        serde_yaml::from_reader(file).expect("Failed to parse config YAML")
    }
}
