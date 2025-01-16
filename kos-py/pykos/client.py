"""KOS client."""

import grpc

from pykos.services.actuator import ActuatorServiceClient
from pykos.services.imu import IMUServiceClient
from pykos.services.inference import InferenceServiceClient
from pykos.services.process_manager import ProcessManagerServiceClient


class KOS:
    """KOS client.

    Args:
        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
        port (int, optional): Port of the robot running KOS. Defaults to 50051.

    Attributes:
        imu (IMUServiceClient): Client for the IMU service.
    """

    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
        self.ip = ip
        self.port = port
        self.channel = grpc.insecure_channel(f"{self.ip}:{self.port}")
        self.imu = IMUServiceClient(self.channel)
        self.actuator = ActuatorServiceClient(self.channel)
        self.process_manager = ProcessManagerServiceClient(self.channel)
        self.inference = InferenceServiceClient(self.channel)

    def close(self) -> None:
        """Close the gRPC channel."""
        self.channel.close()
