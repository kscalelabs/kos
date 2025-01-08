"""IMU service client."""

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import imu_pb2, imu_pb2_grpc


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
        return (
            f"EulerAngles("
            f"roll={self.roll}, pitch={self.pitch}, yaw={self.yaw}, "
            f"error={self.error})"
        )

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
        return (
            f"Quaternion("
            f"x={self.x}, y={self.y}, z={self.z}, w={self.w}, "
            f"error={self.error})"
        )

    def __repr__(self) -> str:
        return self.__str__()


class IMUServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = imu_pb2_grpc.IMUServiceStub(channel)

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
