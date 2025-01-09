"""IMU service client."""

from typing import Any, Dict, Optional

import grpc
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2
from google.protobuf.duration_pb2 import Duration
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, imu_pb2, imu_pb2_grpc
from kos_protos.imu_pb2 import CalibrateIMUMetadata


class CalibrationStatus:
    IN_PROGRESS = "IN_PROGRESS"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"


class CalibrationMetadata:
    def __init__(self, metadata_any: AnyPb2) -> None:
        self.status: Optional[str] = None
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


class ImuValues:
    def __init__(self, response: imu_pb2.IMUValuesResponse) -> None:
        self.accel_x = response.accel_x
        self.accel_y = response.accel_y
        self.accel_z = response.accel_z
        self.gyro_x = response.gyro_x
        self.gyro_y = response.gyro_y
        self.gyro_z = response.gyro_z
        self.mag_x = response.mag_x if response.HasField("mag_x") else None
        self.mag_y = response.mag_y if response.HasField("mag_y") else None
        self.mag_z = response.mag_z if response.HasField("mag_z") else None
        self.grav_x = response.grav_x if response.HasField("grav_x") else None
        self.grav_y = response.grav_y if response.HasField("grav_y") else None
        self.grav_z = response.grav_z if response.HasField("grav_z") else None
        self.error = response.error if response.HasField("error") else None

    def __str__(self) -> str:
        return (
            f"ImuValues("
            f"accel_x={self.accel_x}, accel_y={self.accel_y}, accel_z={self.accel_z}, "
            f"gyro_x={self.gyro_x}, gyro_y={self.gyro_y}, gyro_z={self.gyro_z}, "
            f"mag_x={self.mag_x}, mag_y={self.mag_y}, mag_z={self.mag_z}, "
            f"error={self.error})"
        )

    def __repr__(self) -> str:
        return self.__str__()


class EulerAngles:
    def __init__(self, response: imu_pb2.EulerAnglesResponse) -> None:
        self.roll = response.roll
        self.pitch = response.pitch
        self.yaw = response.yaw
        self.error = response.error if response.HasField("error") else None

    def __str__(self) -> str:
        return f"EulerAngles(" f"roll={self.roll}, pitch={self.pitch}, yaw={self.yaw}, " f"error={self.error})"

    def __repr__(self) -> str:
        return self.__str__()


class Quaternion:
    def __init__(self, response: imu_pb2.QuaternionResponse) -> None:
        self.x = response.x
        self.y = response.y
        self.z = response.z
        self.w = response.w
        self.error = response.error if response.HasField("error") else None

    def __str__(self) -> str:
        return f"Quaternion(" f"x={self.x}, y={self.y}, z={self.z}, w={self.w}, " f"error={self.error})"

    def __repr__(self) -> str:
        return self.__str__()


class IMUServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = imu_pb2_grpc.IMUServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    def get_imu_values(self) -> ImuValues:
        """Get the latest IMU sensor values.

        Returns:
            ImuValuesResponse: The latest IMU sensor values.
        """
        response = self.stub.GetValues(Empty())
        return ImuValues(response)

    def get_euler_angles(self) -> EulerAngles:
        """Get the latest Euler angles.

        Returns:
            EulerAnglesResponse: The latest Euler angles.
        """
        response = self.stub.GetEuler(Empty())
        return EulerAngles(response)

    def get_quaternion(self) -> Quaternion:
        """Get the latest quaternion.

        Returns:
            QuaternionResponse: The latest quaternion.
        """
        response = self.stub.GetQuaternion(Empty())
        return Quaternion(response)

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

    def calibrate(self) -> CalibrationMetadata:
        """Calibrate the IMU.

        This starts a long-running calibration operation. The operation can be monitored
        using get_calibration_status().

        Returns:
            CalibrationMetadata: Metadata about the calibration operation.
        """
        response = self.stub.Calibrate(Empty())
        return CalibrationMetadata(response.metadata)

    def get_calibration_status(self) -> Optional[str]:
        """Get the status of the IMU calibration.

        Returns:
            Optional[str]: The current calibration status if available.
        """
        response = self.operations_stub.GetOperation(
            operations_pb2.GetOperationRequest(name="operations/calibrate_imu/0")
        )
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status
