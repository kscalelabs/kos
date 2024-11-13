import grpc

from pykos.services.imu import IMUServiceClient
from pykos.services.actuator import ActuatorServiceClient
class KOS:
    """
    KOS client

    Args:
        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
        port (int, optional): Port of the robot running KOS. Defaults to 50051.

    Attributes:
        imu (IMUServiceClient): Client for the IMU service.
    """
    def __init__(self, ip: str = "localhost", port: int = 50051):
        self.ip = ip
        self.port = port
        self.channel = grpc.insecure_channel(f"{self.ip}:{self.port}")
        self.imu = IMUServiceClient(self.channel)
        self.actuator = ActuatorServiceClient(self.channel)

    def close(self):
        """
        Close the gRPC channel.
        """
        self.channel.close()
