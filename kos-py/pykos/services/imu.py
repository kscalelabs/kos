"""IMU service client."""

from typing import Any, Dict

import grpc
from google.longrunning import operations_pb2_grpc
from google.protobuf.duration_pb2 import Duration
from google.protobuf.empty_pb2 import Empty
from kos_protos import common_pb2, imu_pb2, imu_pb2_grpc


class CalibrationStatus:
    IN_PROGRESS = "IN_PROGRESS"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"


def _duration_from_seconds(seconds: float) -> Duration:
    """Convert seconds to Duration proto."""
    duration = Duration()
    duration.seconds = int(seconds)
    duration.nanos = int((seconds - int(seconds)) * 1e9)
    return duration


class IMUServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = imu_pb2_grpc.IMUServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    def get_imu_values(self) -> imu_pb2.IMUValuesResponse:
        """Get the latest IMU sensor values.

        Returns:
            ImuValuesResponse: The latest IMU sensor values.
        """
        return self.stub.GetValues(Empty())

    def get_imu_advanced_values(self) -> imu_pb2.IMUAdvancedValuesResponse:
        """Get the latest IMU advanced values.

        Returns:
            ImuAdvancedValuesResponse: The latest IMU advanced values.
        """
        return self.stub.GetAdvancedValues(Empty())

    def get_euler_angles(self) -> imu_pb2.EulerAnglesResponse:
        """Get the latest Euler angles.

        Returns:
            EulerAnglesResponse: The latest Euler angles.
        """
        return self.stub.GetEuler(Empty())

    def get_quaternion(self) -> imu_pb2.QuaternionResponse:
        """Get the latest quaternion.

        Returns:
            QuaternionResponse: The latest quaternion.
        """
        return self.stub.GetQuaternion(Empty())

    def zero(self, duration: float = 1.0, **kwargs: Dict[str, Any]) -> common_pb2.ActionResponse:
        """Zero the IMU.

        Args:
            duration: Duration in seconds for zeroing operation
            **kwargs: Additional zeroing parameters that may include:
                     max_retries: Maximum number of retries
                     max_angular_error: Maximum angular error during zeroing
                     max_velocity: Maximum velocity during zeroing
                     max_acceleration: Maximum acceleration during zeroing

        Returns:
            ActionResponse: The response from the zero operation.
        """
        config = {
            "duration": _duration_from_seconds(duration),
            "max_retries": kwargs.get("max_retries"),
            "max_angular_error": kwargs.get("max_angular_error"),
            "max_velocity": kwargs.get("max_velocity"),
            "max_acceleration": kwargs.get("max_acceleration"),
        }

        config = {k: v for k, v in config.items() if v is not None}

        request = imu_pb2.ZeroIMURequest(**config)
        return self.stub.Zero(request)

    def calibrate(self) -> imu_pb2.CalibrateIMUResponse:
        """Calibrate the IMU.

        This starts a long-running calibration operation. The operation can be monitored
        using get_calibration_status().

        Returns:
            CalibrationMetadata: Metadata about the calibration operation.
        """
        return self.stub.Calibrate(Empty())
