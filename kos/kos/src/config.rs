use robstride::MotorType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
