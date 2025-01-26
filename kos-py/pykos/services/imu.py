"""IMU service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.longrunning import operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2
from google.protobuf.duration_pb2 import Duration
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, imu_pb2, imu_pb2_grpc
from kos_protos.imu_pb2 import (
    CalibrateIMUMetadata,
    IMUAdvancedValuesResponse,
    IMUValuesResponse,
    QuaternionResponse,
)


class EulerAnglesResponse(TypedDict):
    """TypedDict containing orientation in Euler angles.

    A dictionary type containing the IMU's orientation expressed as Euler angles
    in the roll-pitch-yaw convention.

    Fields:
        roll: Rotation around X-axis in radians
        pitch: Rotation around Y-axis in radians
        yaw: Rotation around Z-axis in radians
        error: Optional error information if the calculation failed
    """

    roll: float
    pitch: float
    yaw: float
    error: NotRequired[common_pb2.Error | None]


class ZeroIMUResponse(TypedDict):
    """TypedDict containing response from IMU zeroing operation.

    A dictionary type indicating whether the IMU zeroing operation succeeded.
    Zeroing sets the current orientation as the reference orientation.

    Fields:
        success: Whether the zeroing operation completed successfully
        error: Optional error information if the operation failed
    """

    success: bool
    error: NotRequired[common_pb2.Error | None]


class ActionResponse(TypedDict):
    success: bool
    error: NotRequired[common_pb2.Error]


class CalibrateIMUResponse(TypedDict):
    name: str
    metadata: AnyPb2


class ZeroIMURequest(TypedDict):
    """Parameters for zeroing the IMU.

    Fields:
        max_retries: Optional maximum number of retries
        max_angular_error: Optional maximum angular error during zeroing
        max_velocity: Optional maximum velocity during zeroing
        max_acceleration: Optional maximum acceleration during zeroing
    """

    max_retries: NotRequired[int]
    max_angular_error: NotRequired[float]
    max_velocity: NotRequired[float]
    max_acceleration: NotRequired[float]


class CalibrationStatus:
    IN_PROGRESS = "IN_PROGRESS"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"


class CalibrationMetadata:
    def __init__(self, metadata_any: AnyPb2) -> None:
        self.status: str | None = None
        self.decode_metadata(metadata_any)

    def decode_metadata(self, metadata_any: AnyPb2) -> None:
        metadata = CalibrateIMUMetadata()
        if metadata_any.Is(CalibrateIMUMetadata.DESCRIPTOR):
            metadata_any.Unpack(metadata)
            self.status = metadata.status if metadata.HasField("status") else None

    def __str__(self) -> str:
        return f"CalibrationMetadata(status={self.status})"

    def __repr__(self) -> str:
        return self.__str__()


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

    def get_imu_values(self) -> IMUValuesResponse:
        """Get the latest IMU sensor values.

        Returns:
            IMUValuesResponse containing:
            - accel_x/y/z: Acceleration values in m/s²
            - gyro_x/y/z: Angular velocity values in rad/s
            - mag_x/y/z: Optional magnetic field measurements
            - error: Optional error information if the reading failed
        """
        return self.stub.GetValues(Empty())

    def get_imu_advanced_values(self) -> IMUAdvancedValuesResponse:
        """Get the latest IMU advanced values.

        Returns:
            IMUAdvancedValuesResponse containing:
            - linear_acceleration_x/y/z: Gravity-compensated acceleration in m/s²
            - angular_velocity_x/y/z: Filtered angular velocity in rad/s
            - error: Optional error information if the processing failed
        """
        return self.stub.GetAdvancedValues(Empty())

    def get_euler_angles(self) -> imu_pb2.EulerAnglesResponse:
        """Get the latest Euler angles.

        Returns:
            EulerAnglesResponse containing:
            - roll: Rotation around X-axis in radians
            - pitch: Rotation around Y-axis in radians
            - yaw: Rotation around Z-axis in radians
            - error: Optional error information if the calculation failed
        """
        return self.stub.GetEuler(Empty())

    def get_quaternion(self) -> QuaternionResponse:
        """Get the latest quaternion.

        Returns:
            QuaternionResponse containing:
            - w: Scalar component
            - x/y/z: Vector components
            - error: Optional error information if the calculation failed
        """
        return self.stub.GetQuaternion(Empty())

    def zero(self, duration: float = 1.0, **kwargs: Unpack[ZeroIMURequest]) -> common_pb2.ActionResponse:
        """Zero the IMU.

        Example:
            >>> zero(duration=1.0,
            ...     max_retries=3,
            ...     max_angular_error=1.0,
            ...     max_velocity=1.0,
            ...     max_acceleration=1.0
            ... )

        Args:
            duration: Duration in seconds for zeroing operation
            **kwargs: Additional zeroing parameters that may include:
                     max_retries: Maximum number of retries
                     max_angular_error: Maximum angular error during zeroing
                     max_velocity: Maximum velocity during zeroing
                     max_acceleration: Maximum acceleration during zeroing

        Returns:
            ActionResponse indicating if the zeroing operation was successful
        """
        request = imu_pb2.ZeroIMURequest(duration=_duration_from_seconds(duration), **kwargs)
        return self.stub.Zero(request)

    def calibrate(self) -> imu_pb2.CalibrateIMUResponse:
        """Calibrate the IMU.

        This starts a long-running calibration operation. The operation can be monitored
        using get_calibration_status().

        Returns:
            CalibrateIMUResponse containing operation name and metadata about the calibration.
        """
        return self.stub.Calibrate(Empty())
