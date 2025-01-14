"""Defines a dummy test."""

import grpc
import pykos
import pytest


def test_dummy() -> None:
    assert True


def test_pykos() -> None:
    if not is_server_running("127.0.0.1:50051"):
        pytest.skip("No active gRPC server at 127.0.0.1:50051")
    client = pykos.KOS("127.0.0.1")

    # Tests configuring the actuator.
    actuator_response = client.actuator.configure_actuator(actuator_id=1)
    assert actuator_response.success

    # Tests getting the actuator state.
    actuator_state = client.actuator.get_actuators_state(actuator_ids=[1])
    assert actuator_state[0].actuator_id == 1

    # Tests the IMU endpoint.
    imu_response = client.imu.get_imu_values()
    assert imu_response.accel_x is not None
    client.imu.get_imu_advanced_values()
    client.imu.get_euler_angles()
    client.imu.get_quaternion()
    client.imu.calibrate()


def is_server_running(address: str) -> bool:
    try:
        channel = grpc.insecure_channel(address)
        grpc.channel_ready_future(channel).result(timeout=1)
        return True
    except grpc.FutureTimeoutError:
        return False
