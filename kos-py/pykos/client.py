"""KOS client."""

import asyncio
import warnings
from typing import Any

import grpc
import grpc.aio
import nest_asyncio

from pykos.services.actuator import ActuatorServiceClient
from pykos.services.imu import IMUServiceClient
from pykos.services.inference import InferenceServiceClient
from pykos.services.led_matrix import LEDMatrixServiceClient
from pykos.services.process_manager import ProcessManagerServiceClient
from pykos.services.sim import SimServiceClient
from pykos.services.sound import SoundServiceClient
from pykos.services.policy import PolicyServiceClient


class KOS:
    """KOS client.

    Args:
        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
        port (int, optional): Port of the robot running KOS. Defaults to 50051.

    Attributes:
        imu (IMUServiceClient): Client for the IMU service.
    """

    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
        self.is_open = False
        self.ip = ip
        self.port = port
        self._channel: grpc.aio.Channel | None = None
        self._imu: IMUServiceClient | None = None
        self._actuator: ActuatorServiceClient | None = None
        self._led_matrix: LEDMatrixServiceClient | None = None
        self._sound: SoundServiceClient | None = None
        self._process_manager: ProcessManagerServiceClient | None = None
        self._inference: InferenceServiceClient | None = None
        self._sim: SimServiceClient | None = None
        self._policy: PolicyServiceClient | None = None

    @property
    def imu(self) -> IMUServiceClient:
        if self._imu is None:
            self.connect()
        if self._imu is None:
            raise RuntimeError("IMU client not initialized! Must call `connect()` manually.")
        return self._imu

    @property
    def actuator(self) -> ActuatorServiceClient:
        if self._actuator is None:
            self.connect()
        if self._actuator is None:
            raise RuntimeError("Actuator client not initialized! Must call `connect()` manually.")
        return self._actuator

    @property
    def led_matrix(self) -> LEDMatrixServiceClient:
        if self._led_matrix is None:
            self.connect()
        if self._led_matrix is None:
            raise RuntimeError("LED Matrix client not initialized! Must call `connect()` manually.")
        return self._led_matrix

    @property
    def sound(self) -> SoundServiceClient:
        if self._sound is None:
            self.connect()
        if self._sound is None:
            raise RuntimeError("Sound client not initialized! Must call `connect()` manually.")
        return self._sound

    @property
    def process_manager(self) -> ProcessManagerServiceClient:
        if self._process_manager is None:
            self.connect()
        if self._process_manager is None:
            raise RuntimeError("Process Manager client not initialized! Must call `connect()` manually.")
        return self._process_manager

    @property
    def policy(self) -> PolicyServiceClient:
        if self._policy is None:
            self.connect()
        if self._policy is None:
            raise RuntimeError("Policy client not initialized! Must call `connect()` manually.")
        return self._policy

    @property
    def inference(self) -> InferenceServiceClient:
        if self._inference is None:
            self.connect()
        if self._inference is None:
            raise RuntimeError("Inference client not initialized! Must call `connect()` manually.")
        return self._inference

    @property
    def sim(self) -> SimServiceClient:
        if self._sim is None:
            self.connect()
        if self._sim is None:
            raise RuntimeError("Sim client not initialized! Must call `connect()` manually.")
        return self._sim

    def connect(self) -> None:
        """Connect to the gRPC server and initialize service clients."""
        # Patch asyncio to allow running async code in a synchronous context.
        nest_asyncio.apply()

        self._channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
        self._imu = IMUServiceClient(self._channel)
        self._actuator = ActuatorServiceClient(self._channel)
        self._led_matrix = LEDMatrixServiceClient(self._channel)
        self._sound = SoundServiceClient(self._channel)
        self._process_manager = ProcessManagerServiceClient(self._channel)
        self._policy = PolicyServiceClient(self._channel) 
        self._inference = InferenceServiceClient(self._channel)
        self._sim = SimServiceClient(self._channel)

    async def close(self) -> None:
        """Close the gRPC channel."""
        if self._channel is not None:
            await self._channel.close()

    def __enter__(self) -> "KOS":
        self.connect()
        self.is_open = True
        return self

    def __exit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
        asyncio.run(self.close())
        self.is_open = False

    async def __aenter__(self) -> "KOS":
        self.connect()
        self.is_open = True
        return self

    async def __aexit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
        await self.close()
        self.is_open = False

    def __del__(self) -> None:
        if self.is_open:
            warnings.warn("KOS client was not closed before deletion. This may cause resource leaks.")
