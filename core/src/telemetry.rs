// TODO: Implement telemetry.
// General idea - MQTT for the robot, where serial of the robot is a topic.
// Mosquitto is the broker which will pass messages to InfluxDB
// We log desired vs actual joint angles (torque/velocity/position if applicable),
// as well as IMU data.