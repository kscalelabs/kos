pub trait Actuator: Send + Sync {
    fn set_position(&self, position: f32) -> Result<(), Box<dyn std::error::Error>>;
    fn get_position(&self) -> Result<f32, Box<dyn std::error::Error>>;
}

pub trait IMU: Send + Sync {
    fn read_acceleration(&self) -> Result<(f32, f32, f32), Box<dyn std::error::Error>>;
    fn read_gyroscope(&self) -> Result<(f32, f32, f32), Box<dyn std::error::Error>>;
}

pub trait Initializer {
    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>>;
}
