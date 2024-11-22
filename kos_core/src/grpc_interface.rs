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
