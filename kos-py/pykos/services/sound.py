"""Sound service client."""

from typing import Generator, Iterator, NotRequired, TypedDict, Unpack

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sound_pb2, sound_pb2_grpc


class AudioCapability(TypedDict):
    """Information about audio capabilities.

    Args:
        sample_rates: List of supported sample rates (e.g., 44100, 48000)
        bit_depths: List of supported bit depths (e.g., 16, 24, 32)
        channels: List of supported channel counts (e.g., 1, 2)
        available: Whether this capability is available
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
            AudioInfo containing playback and recording capabilities.
        """
        return self.stub.GetAudioInfo(Empty())

    def play_audio(self, audio_iterator: Iterator[bytes], **kwargs: Unpack[AudioConfig]) -> common_pb2.ActionResponse:
        """Stream PCM audio data to the speaker.

        Args:
            audio_iterator: Iterator yielding chunks of PCM audio data
            **kwargs: Audio configuration parameters
                sample_rate: Sample rate in Hz (e.g., 44100)
                bit_depth: Bit depth (e.g., 16)
                channels: Number of channels (1 for mono, 2 for stereo)

        Returns:
            ActionResponse indicating success/failure of the playback operation.

        Example:
            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=2)
            >>> with open('audio.raw', 'rb') as f:
            ...     def chunks():
            ...         while chunk := f.read(4096):
            ...             yield chunk
            ...     response = client.play_audio(chunks(), config)
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

        Args:
            duration_ms: Recording duration in milliseconds (0 for continuous)
            **kwargs: Audio configuration parameters
                sample_rate: Sample rate in Hz (e.g., 44100)
                bit_depth: Bit depth (e.g., 16)
                channels: Number of channels (1 for mono, 2 for stereo)

        Yields:
            Chunks of PCM audio data.

        Example:
            >>> config = AudioConfig(sample_rate=44100, bit_depth=16, channels=1)
            >>> with open('recording.raw', 'wb') as f:
            ...     for chunk in client.record_audio(duration_ms=5000, **config):
            ...         f.write(chunk)
        """
        request = sound_pb2.RecordAudioRequest(
            config=sound_pb2.AudioConfig(**kwargs),
            duration_ms=duration_ms,
        )

        for response in self.stub.RecordAudio(request):
            if response.HasField("error"):
                raise RuntimeError(f"Recording error: {response.error}")
            yield response.audio_data

    def stop_recording(self) -> common_pb2.ActionResponse:
        """Stop an ongoing recording session.

        Returns:
            ActionResponse indicating success/failure of the stop operation.
        """
        return self.stub.StopRecording(Empty())
