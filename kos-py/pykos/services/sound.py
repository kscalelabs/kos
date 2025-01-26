"""Sound service client."""

from typing import Generator, Iterator, NotRequired, TypedDict

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sound_pb2_grpc
from kos_protos.sound_pb2 import (
    AudioConfig as ProtoAudioConfig,
    GetAudioInfoResponse,
    PlayAudioRequest,
    RecordAudioRequest,
)


class AudioCapability(TypedDict):
    """TypedDict containing information about audio capabilities.

    A dictionary type describing the supported audio configurations
    and current availability of the audio system.

    Fields:
        sample_rates: List of supported sampling rates in Hz
        bit_depths: List of supported bit depths
        channels: List of supported channel counts
        available: Whether the audio system is currently available
    """

    sample_rates: list[int]
    bit_depths: list[int]
    channels: list[int]
    available: bool


class AudioInfo(TypedDict):
    """Information about audio system capabilities.

    Args:
        playback: Playback capabilities
        recording: Recording capabilities
        error: Optional error information
    """

    playback: AudioCapability
    recording: AudioCapability
    error: NotRequired[common_pb2.Error | None]


class AudioConfig(TypedDict):
    """Audio configuration parameters.

    Fields:
        sample_rate: Sample rate in Hz (e.g., 44100)
        bit_depth: Bit depth (e.g., 16)
        channels: Number of channels (1 for mono, 2 for stereo)
    """

    sample_rate: int
    bit_depth: int
    channels: int


class ActionResponse(TypedDict):
    """Response indicating success/failure of an action."""

    success: bool
    error: NotRequired[common_pb2.Error | None]


class SoundServiceClient:
    """Client for the sound service.

    This client provides methods to interact with the audio system,
    including playback and recording capabilities.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the sound service client.

        Args:
            channel: gRPC channel for communication with the service
        """
        self.stub = sound_pb2_grpc.SoundServiceStub(channel)

    def get_audio_info(self) -> GetAudioInfoResponse:
        """Get information about audio system capabilities.

        Returns:
            GetAudioInfoResponse containing:
            - playback: Playback capabilities (sample rates, bit depths, channels)
            - recording: Recording capabilities (sample rates, bit depths, channels)
            - error: Optional error information
        """
        return self.stub.GetAudioInfo(Empty())

    def play_audio(
        self,
        audio_iterator: Iterator[bytes],
        sample_rate: int,
        bit_depth: int,
        channels: int,
    ) -> common_pb2.ActionResponse:
        """Play audio data through the audio system.

        Args:
            audio_iterator: Iterator yielding audio data chunks
            sample_rate: Sample rate in Hz (e.g., 44100)
            bit_depth: Bit depth (e.g., 16)
            channels: Number of channels (1 for mono, 2 for stereo)

        Returns:
            ActionResponse indicating if the playback was successful
        """

        def request_iterator() -> Generator[PlayAudioRequest, None, None]:
            # First message includes config
            yield PlayAudioRequest(
                config=ProtoAudioConfig(
                    sample_rate=sample_rate,
                    bit_depth=bit_depth,
                    channels=channels,
                )
            )
            # Subsequent messages contain audio data
            for chunk in audio_iterator:
                yield PlayAudioRequest(audio_data=chunk)

        return self.stub.PlayAudio(request_iterator())

    def record_audio(
        self,
        duration_ms: int = 0,
        sample_rate: int = 44100,
        bit_depth: int = 16,
        channels: int = 1,
    ) -> Generator[bytes, None, None]:
        """Record audio from the audio system.

        Args:
            duration_ms: Duration to record in milliseconds (0 for continuous)
            sample_rate: Sample rate in Hz (default: 44100)
            bit_depth: Bit depth (default: 16)
            channels: Number of channels (default: 1 for mono)

        Returns:
            Generator yielding recorded audio data chunks
        """
        request = RecordAudioRequest(
            config=ProtoAudioConfig(
                sample_rate=sample_rate,
                bit_depth=bit_depth,
                channels=channels,
            ),
            duration_ms=duration_ms,
        )
        for response in self.stub.RecordAudio(request):
            if response.HasField("error"):
                raise RuntimeError(f"Recording error: {response.error}")
            yield response.audio_data

    def stop_recording(self) -> common_pb2.ActionResponse:
        """Stop the current audio recording.

        Returns:
            ActionResponse indicating if the recording was stopped successfully
        """
        return self.stub.StopRecording(Empty())
