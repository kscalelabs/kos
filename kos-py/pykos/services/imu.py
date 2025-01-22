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
    """TypedDict containing basic IMU sensor measurements.

    A dictionary type containing raw accelerometer, gyroscope, and optional
    magnetometer readings from the IMU sensor.

    Fields:
        accel_x: Acceleration along X-axis in m/s²
        accel_y: Acceleration along Y-axis in m/s²
        accel_z: Acceleration along Z-axis in m/s²
        gyro_x: Angular velocity around X-axis in rad/s
        gyro_y: Angular velocity around Y-axis in rad/s
        gyro_z: Angular velocity around Z-axis in rad/s
        mag_x: Optional magnetic field strength along X-axis
        mag_y: Optional magnetic field strength along Y-axis
        mag_z: Optional magnetic field strength along Z-axis
        error: Optional error information if the reading failed
    """

    accel_x: float
    accel_y: float
    accel_z: float
    gyro_x: float
    gyro_y: float
    gyro_z: float
    mag_x: NotRequired[float | None]
    mag_y: NotRequired[float | None]
    mag_z: NotRequired[float | None]
    error: NotRequired[common_pb2.Error | None]


class IMUAdvancedValuesResponse(TypedDict):
    """TypedDict containing processed IMU measurements.

    A dictionary type containing filtered and processed IMU readings,
    including linear accelerations and angular velocities with gravity compensation.

    Fields:
        linear_acceleration_x: Gravity-compensated acceleration along X-axis in m/s²
        linear_acceleration_y: Gravity-compensated acceleration along Y-axis in m/s²
        linear_acceleration_z: Gravity-compensated acceleration along Z-axis in m/s²
        angular_velocity_x: Filtered angular velocity around X-axis in rad/s
        angular_velocity_y: Filtered angular velocity around Y-axis in rad/s
        angular_velocity_z: Filtered angular velocity around Z-axis in rad/s
        error: Optional error information if the processing failed
    """

    linear_acceleration_x: float
    linear_acceleration_y: float
    linear_acceleration_z: float
    angular_velocity_x: float
    angular_velocity_y: float
    angular_velocity_z: float
    error: NotRequired[common_pb2.Error | None]


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


class QuaternionResponse(TypedDict):
    """TypedDict containing orientation as quaternion.

    A dictionary type containing the IMU's orientation expressed as a unit quaternion,
    which provides a singularity-free representation of orientation.

    Fields:
        w: Scalar component of the quaternion
        x: X component of the quaternion's vector part
        y: Y component of the quaternion's vector part
        z: Z component of the quaternion's vector part
        error: Optional error information if the calculation failed
    """

    w: float
    x: float
    y: float
    z: float
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
            IMUValuesResponse is a dictionary containing raw sensor measurements where:
            - 'accel_x/y/z' contain acceleration values in m/s²
            - 'gyro_x/y/z' contain angular velocity values in rad/s
            - 'mag_x/y/z' optionally contain magnetic field measurements
            - 'error' contains any error information if the reading failed
        """
        return self.stub.GetValues(Empty())

    def get_imu_advanced_values(self) -> IMUAdvancedValuesResponse:
        """Get the latest IMU advanced values.

        Returns:
            IMUAdvancedValuesResponse is a dictionary containing processed measurements where:
            - 'linear_acceleration_x/y/z' contain gravity-compensated acceleration in m/s²
            - 'angular_velocity_x/y/z' contain filtered angular velocity in rad/s
            - 'error' contains any error information if the processing failed
        """
        return self.stub.GetAdvancedValues(Empty())

    def get_euler_angles(self) -> EulerAnglesResponse:
        """Get the latest Euler angles.

        Returns:
            EulerAnglesResponse is a dictionary containing orientation angles where:
            - 'roll' contains rotation around X-axis in radians
            - 'pitch' contains rotation around Y-axis in radians
            - 'yaw' contains rotation around Z-axis in radians
            - 'error' contains any error information if the calculation failed
        """
        return self.stub.GetEuler(Empty())

    def get_quaternion(self) -> QuaternionResponse:
        """Get the latest quaternion.

        Returns:
            QuaternionResponse is a dictionary containing orientation as quaternion where:
            - 'w' contains the scalar component
            - 'x/y/z' contain the vector components
            - 'error' contains any error information if the calculation failed
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
            ActionResponse is a dictionary where 'success' indicates if the zeroing operation
            was successful, and 'error' contains any error information if the operation failed.
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
