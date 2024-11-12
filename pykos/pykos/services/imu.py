from kos import imu_pb2_grpc, imu_pb2
from google.protobuf.empty_pb2 import Empty

class ImuValues:
    def __init__(self, response: imu_pb2.IMUValuesResponse):
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
    def __str__(self):
        return f"ImuValues(accel_x={self.accel_x}, accel_y={self.accel_y}, accel_z={self.accel_z}, gyro_x={self.gyro_x}, gyro_y={self.gyro_y}, gyro_z={self.gyro_z}, mag_x={self.mag_x}, mag_y={self.mag_y}, mag_z={self.mag_z}, error={self.error})"
    def __repr__(self):
        return self.__str__()

class IMUServiceClient:
    def __init__(self, channel):
        self.stub = imu_pb2_grpc.IMUServiceStub(channel)

    def get_imu_values(self):
        """
        Get the latest IMU sensor values.

        Returns:
            ImuValuesResponse: The latest IMU sensor values.
        """
        response = self.stub.GetValues(Empty())
        return ImuValues(response)
    