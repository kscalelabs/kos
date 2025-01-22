"""KOS client."""

from typing import Any

import grpc
import grpc.aio

from pykos.services.actuator import ActuatorServiceClient
from pykos.services.imu import IMUServiceClient
from pykos.services.inference import InferenceServiceClient
from pykos.services.led_matrix import LEDMatrixServiceClient
from pykos.services.process_manager import ProcessManagerServiceClient
from pykos.services.sim import SimServiceClient
from pykos.services.sound import SoundServiceClient


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
        self.channel = None
        self.imu = None
        self.actuator = None
        self.led_matrix = None
        self.sound = None
        self.process_manager = None
        self.inference = None
        self.sim = None

    async def connect(self) -> None:
        """Connect to the gRPC server and initialize service clients."""
        self.channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
        self.imu = IMUServiceClient(self.channel)
        self.actuator = ActuatorServiceClient(self.channel)
        self.led_matrix = LEDMatrixServiceClient(self.channel)
        self.sound = SoundServiceClient(self.channel)
        self.process_manager = ProcessManagerServiceClient(self.channel)
        self.inference = InferenceServiceClient(self.channel)
        self.sim = SimServiceClient(self.channel)

    async def close(self) -> None:
        """Close the gRPC channel."""
        if self.channel is not None:
            await self.channel.close()

    async def __aenter__(self) -> "KOS":
        await self.connect()
        return self

    async def __aexit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
        await self.close()
