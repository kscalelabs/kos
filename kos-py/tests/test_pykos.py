"""Defines a dummy test."""

import grpc
import pytest

import pykos


def test_dummy() -> None:
    assert True


@pytest.mark.asyncio
async def test_pykos() -> None:
    # In order to test this client, you should run the stub KOS server.
    # This can be done from the parent directory with `cargo run --bin kos-stub`
    if not _is_server_running("127.0.0.1:50051"):
        pytest.skip("No active gRPC server at 127.0.0.1:50051")

    # Tests configuring the actuator.
    async with pykos.KOS("127.0.0.1") as client:
        actuator_response = await client.actuator.configure_actuator(actuator_id=1)
        assert actuator_response.success

        # Tests getting the actuator state.
        actuator_state = await client.actuator.get_actuators_state(actuator_ids=[1])
        assert actuator_state.states[0].actuator_id == 1

        # Tests the IMU endpoints.
        imu_response = await client.imu.get_imu_values()
        assert imu_response.accel_x is not None
        await client.imu.get_imu_advanced_values()
        await client.imu.get_euler_angles()
        await client.imu.get_quaternion()
        await client.imu.calibrate()
        zero_response = await client.imu.zero(duration=1.0, max_retries=1, max_angular_error=1.0)
        assert zero_response.success

        # Tests the K-Clip endpoints.
        start_kclip_response = await client.process_manager.start_kclip(action="start")
        assert start_kclip_response.clip_uuid is not None
        stop_kclip_response = await client.process_manager.stop_kclip()
        assert stop_kclip_response.clip_uuid is not None


def _is_server_running(address: str) -> bool:
    try:
        channel = grpc.insecure_channel(address)
        grpc.channel_ready_future(channel).result(timeout=1)
        return True
    except grpc.FutureTimeoutError:
        return False
