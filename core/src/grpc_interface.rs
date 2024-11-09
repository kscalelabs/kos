pub mod robot_control {
    tonic::include_proto!("kscaleos.control");
}

pub use robot_control::*;
