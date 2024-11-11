use std::env;
use std::path::PathBuf;

fn main() {
    // Path to the Protobuf files
    let proto_root = "../proto";

    // Where to output the compiled Rust files
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // List of Protobuf files
    let protos = [
        "kos/common.proto",
        "kos/actuator.proto",
        "kos/imu.proto",
        "kos/inference.proto",
        "kos/process_manager.proto",
        "kos/system.proto",
        "google/longrunning/operations.proto",
    ];

    let includes = [proto_root, &format!("{}/googleapis", proto_root)];

    // Create the output directory
    std::fs::create_dir_all(out_dir.join("kos"))
        .expect("Failed to create output directory");

    // Configure and compile Protobuf files
    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir.join("kos"))
        .compile_protos(&protos, &includes)
        .expect("Failed to compile protos");

    // Re-run the build script if any of the proto files change
    for proto in &protos {
        println!("cargo:rerun-if-changed={}/kos/{}", proto_root, proto);
    }
    println!("cargo:rerun-if-changed={}", proto_root);
}
