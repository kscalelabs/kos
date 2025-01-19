"""IMU service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.longrunning import operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2
from google.protobuf.duration_pb2 import Duration
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, imu_pb2, imu_pb2_grpc
from kos_protos.imu_pb2 import CalibrateIMUMetadata


class IMUValuesResponse(TypedDict):
    acceleration_x: float
    acceleration_y: float
    acceleration_z: float
    gyroscope_x: float
    gyroscope_y: float
    gyroscope_z: float
    magnetometer_x: float
    magnetometer_y: float
    magnetometer_z: float


class IMUAdvancedValuesResponse(TypedDict):
    linear_acceleration_x: float
    linear_acceleration_y: float
    linear_acceleration_z: float
    gravity_x: float
    gravity_y: float
    gravity_z: float
    rotation_rate_x: float
    rotation_rate_y: float
    rotation_rate_z: float


class EulerAnglesResponse(TypedDict):
    roll: float
    pitch: float
    yaw: float


class QuaternionResponse(TypedDict):
    w: float
    x: float
    y: float
    z: float


class ActionResponse(TypedDict):
    success: bool
    error: NotRequired[common_pb2.Error]


class CalibrateIMUResponse(TypedDict):
    name: str
    metadata: AnyPb2


class ZeroIMURequest(TypedDict):
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
            IMUValuesResponse: The latest IMU sensor values including acceleration,
            gyroscope, and magnetometer readings on x, y, z axes.
        """
        return self.stub.GetValues(Empty())

    def get_imu_advanced_values(self) -> IMUAdvancedValuesResponse:
        """Get the latest IMU advanced values.

        Returns:
            IMUAdvancedValuesResponse: The latest IMU advanced values including linear acceleration,
            gravity, and rotation rate on x, y, z axes.
        """
        return self.stub.GetAdvancedValues(Empty())

    def get_euler_angles(self) -> EulerAnglesResponse:
        """Get the latest Euler angles.

        Returns:
            EulerAnglesResponse: The latest Euler angles (roll, pitch, yaw).
        """
        return self.stub.GetEuler(Empty())

    def get_quaternion(self) -> QuaternionResponse:
        """Get the latest quaternion.

        Returns:
            QuaternionResponse: The latest quaternion values (w, x, y, z).
        """
        return self.stub.GetQuaternion(Empty())

    def zero(self, duration: float = 1.0, **kwargs: Unpack[ZeroIMURequest]) -> ActionResponse:
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
            ActionResponse: Response indicating success/failure of the zero operation.
        """
        request = imu_pb2.ZeroIMURequest(duration=_duration_from_seconds(duration), **kwargs)
        return self.stub.Zero(request)

    def calibrate(self) -> CalibrateIMUResponse:
        """Calibrate the IMU.

        This starts a long-running calibration operation. The operation can be monitored
        using get_calibration_status().

        Returns:
            CalibrateIMUResponse: Response containing operation name and metadata about the calibration.
        """
        return self.stub.Calibrate(Empty())
