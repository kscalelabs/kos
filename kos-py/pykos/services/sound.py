"""Sound service client."""

from typing import Generator, Iterator, NotRequired, TypedDict, Unpack

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sound_pb2, sound_pb2_grpc


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

    Args:
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


class RecordAudioResponse(TypedDict):
    """Response containing recorded audio data."""

    audio_data: bytes
    error: NotRequired[common_pb2.Error | None]


class SoundServiceClient:
    """Client for the SoundService.

    This service allows playing audio through speakers and recording from microphones.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the sound service client.

        Args:
            channel: gRPC channel to use for communication.
        """
        self.stub = sound_pb2_grpc.SoundServiceStub(channel)

    def get_audio_info(self) -> AudioInfo:
        """Get information about audio capabilities.

        Returns:
            AudioInfo is a dictionary where:
            - 'playback' contains an AudioCapability dictionary describing playback capabilities:
            - 'recording' contains an AudioCapability dictionary describing recording capabilities
            - 'error' contains any error information if the query failed
        """
        return self.stub.GetAudioInfo(Empty())

    def play_audio(self, audio_iterator: Iterator[bytes], **kwargs: Unpack[AudioConfig]) -> ActionResponse:
        """Stream PCM audio data to the speaker.

        Example:
            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2)
            >>> with open('audio.raw', 'rb') as f:
            ...     def chunks():
            ...         while chunk := f.read(4096):
            ...             yield chunk
            ...     response = client.play_audio(chunks(), **config)

        Args:
            audio_iterator: Iterator yielding chunks of PCM audio data
            **kwargs: Audio configuration parameters:
                     sample_rate: Sample rate in Hz (e.g., 44100)
                     bit_depth: Bit depth (e.g., 16)
                     channels: Number of channels (1 for mono, 2 for stereo)

        Returns:
            ActionResponse is a dictionary where 'success' indicates if the playback operation
            was successful, and 'error' contains any error information if the operation failed.
        """

        def request_iterator() -> Generator[sound_pb2.PlayAudioRequest, None, None]:
            # First message includes config
            yield sound_pb2.PlayAudioRequest(
                config=sound_pb2.AudioConfig(**kwargs),
            )
            # Subsequent messages contain audio data
            for chunk in audio_iterator:
                yield sound_pb2.PlayAudioRequest(audio_data=chunk)

        return self.stub.PlayAudio(request_iterator())

    def record_audio(self, duration_ms: int = 0, **kwargs: Unpack[AudioConfig]) -> Generator[bytes, None, None]:
        """Record PCM audio data from the microphone.

        Example:
            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1)
            >>> with open('recording.raw', 'wb') as f:
            ...     for chunk in client.record_audio(duration_ms=5000, **config):
            ...         f.write(chunk)

        Args:
            duration_ms: Recording duration in milliseconds (0 for continuous)
            **kwargs: Audio configuration parameters:
                     sample_rate: Sample rate in Hz (e.g., 44100)
                     bit_depth: Bit depth (e.g., 16)
                     channels: Number of channels (1 for mono, 2 for stereo)

        Yields:
            Chunks of PCM audio data as bytes. If an error occurs during recording,
            a RuntimeError will be raised with the error details.
        """
        request = sound_pb2.RecordAudioRequest(
            config=sound_pb2.AudioConfig(**kwargs),
            duration_ms=duration_ms,
        )

        for response in self.stub.RecordAudio(request):
            if response.HasField("error"):
                raise RuntimeError(f"Recording error: {response.error}")
            yield response.audio_data

    def stop_recording(self) -> ActionResponse:
        """Stop an ongoing recording session.

        Returns:
            ActionResponse is a dictionary where 'success' indicates if the recording was
            stopped successfully, and 'error' contains any error information if the stop failed.
        """
        return self.stub.StopRecording(Empty())
