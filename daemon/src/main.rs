// TODO: Implement daemon.
// This will be the main process that will manage the robot.
// It will run the gRPC server, and, if applicable,
// the runtime loop (e.g. actuator polling, loaded model inference).

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
