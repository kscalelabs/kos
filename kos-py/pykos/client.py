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
from pykos.services.speech import SpeechServiceClient


class KOS:
    """KOS client.

    Args:
        ip (str, optional): IP address of the robot running KOS. Defaults to localhost.
        port (int, optional): Port of the robot running KOS. Defaults to 50051.

    Attributes:
        imu (IMUServiceClient): Client for the IMU service.
        actuator (ActuatorServiceClient): Client for the actuator service.
        led_matrix (LEDMatrixServiceClient): Client for the LED matrix service.
        sound (SoundServiceClient): Client for the sound service.
        process_manager (ProcessManagerServiceClient): Client for the process manager service.
        inference (InferenceServiceClient): Client for the inference service.
        sim (SimServiceClient): Client for the simulation service.
        speech (SpeechServiceClient): Client for the speech service.
    """

    def __init__(self, ip: str = "localhost", port: int = 50051) -> None:
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
        self._speech: SpeechServiceClient | None = None

    @property
    def imu(self) -> IMUServiceClient:
        if self._imu is None:
            raise RuntimeError("IMU client not initialized! Must call __aenter__() first.")
        return self._imu

    @property
    def actuator(self) -> ActuatorServiceClient:
        if self._actuator is None:
            raise RuntimeError("Actuator client not initialized! Must call __aenter__() first.")
        return self._actuator

    @property
    def led_matrix(self) -> LEDMatrixServiceClient:
        if self._led_matrix is None:
            raise RuntimeError("LED Matrix client not initialized! Must call __aenter__() first.")
        return self._led_matrix

    @property
    def sound(self) -> SoundServiceClient:
        if self._sound is None:
            raise RuntimeError("Sound client not initialized! Must call __aenter__() first.")
        return self._sound

    @property
    def process_manager(self) -> ProcessManagerServiceClient:
        if self._process_manager is None:
            raise RuntimeError("Process Manager client not initialized! Must call __aenter__() first.")
        return self._process_manager

    @property
    def inference(self) -> InferenceServiceClient:
        if self._inference is None:
            raise RuntimeError("Inference client not initialized! Must call __aenter__() first.")
        return self._inference

    @property
    def sim(self) -> SimServiceClient:
        if self._sim is None:
            raise RuntimeError("Sim client not initialized! Must call __aenter__() first.")
        return self._sim

    @property
    def speech(self) -> SpeechServiceClient:
        if self._speech is None:
            raise RuntimeError("Speech client not initialized! Must call __aenter__() first.")
        return self._speech

    async def connect(self) -> None:
        """Connect to the gRPC server and initialize service clients."""
        self._channel = grpc.aio.insecure_channel(f"{self.ip}:{self.port}")
        self._process_manager = ProcessManagerServiceClient(self._channel)
        self._imu = IMUServiceClient(self._channel)
        self._actuator = ActuatorServiceClient(self._channel)
        self._led_matrix = LEDMatrixServiceClient(self._channel)
        self._sound = SoundServiceClient(self._channel)
        self._speech = SpeechServiceClient(self._channel)
        self._inference = InferenceServiceClient(self._channel)
        self._sim = SimServiceClient(self._channel)

    async def close(self) -> None:
        """Close the gRPC channel."""
        if self._channel is not None:
            await self._channel.close()

    async def __aenter__(self) -> "KOS":
        await self.connect()
        return self

    async def __aexit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:  # noqa: ANN401
        await self.close()
