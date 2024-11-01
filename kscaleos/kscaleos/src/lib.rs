use robstride::{MotorType, MotorsSupervisor};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod config;
mod runner;
mod state;

fn main() {
    let config = config::Config::new("config.yaml");
    let state = Arc::new(RwLock::new(State::new()));

    // Four motor supervisors, one for each limb.
    let mut motor_supervisors = vec![];
    for i in 0..4 {
        let port_name = format!("/dev/ttyCH341USB{}", i);

        let motor_infos = match i {
            // Left arm.
            0 => HashMap::from([
                (1, MotorType::Type03),
                (2, MotorType::Type03),
                (3, MotorType::Type02),
                (4, MotorType::Type02),
                (5, MotorType::Type01),
            ]),
            // Right arm.
            1 => HashMap::from([
                (1, MotorType::Type03),
                (2, MotorType::Type03),
                (3, MotorType::Type02),
                (4, MotorType::Type02),
                (5, MotorType::Type01),
            ]),
            // Left leg.
            2 => HashMap::from([
                (1, MotorType::Type04),
                (2, MotorType::Type03),
                (3, MotorType::Type03),
                (4, MotorType::Type03),
                (5, MotorType::Type01),
            ]),
            // Right leg.
            3 => HashMap::from([
                (1, MotorType::Type04),
                (2, MotorType::Type03),
                (3, MotorType::Type03),
                (4, MotorType::Type03),
                (5, MotorType::Type01),
            ]),
            _ => {
                panic!("Invalid limb index: {}", i);
            }
        };

        let motor_supervisor =
            MotorsSupervisor::new(port_name.as_str(), &motor_infos, false, 1000.0, true).unwrap();
        motor_supervisors.push(motor_supervisor);
    }
}

struct State {}

impl State {
    fn new() -> Self {}

    fn update(&mut self, _input: SensorData, _commands: Commands) {
        // Update state logic
    }
}

fn read_sensors() -> SensorData {
    // Read from camera and motors
    SensorData { /* fields */ }
}

fn send_commands(_commands: Commands) {
    // Interface with motor drivers
}

struct SensorData {/* fields */}

struct Commands {/* fields */}
