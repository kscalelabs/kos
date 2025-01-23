"""Speech service client."""

from typing import TypedDict, Unpack

import grpc
import grpc.aio

from kos_protos import speech_pb2, speech_pb2_grpc


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


class SpeechServiceClient:
    """Client for the SpeechService.

    This service provides text-to-speech synthesis and speech-to-text transcription.
    """

    def __init__(self, channel: grpc.aio.Channel) -> None:
        """Initialize the speech service client.

        Args:
            channel: gRPC channel to use for communication.
        """
        self.stub = speech_pb2_grpc.SpeechServiceStub(channel)

    async def synthesize(self, text: str, **kwargs: Unpack[AudioConfig]) -> str:
        """Synthesize speech from text.

        Args:
            text: Text to synthesize
            **kwargs: Audio configuration parameters
                sample_rate: Sample rate in Hz (e.g., 44100)
                bit_depth: Bit depth (e.g., 16)
                channels: Number of channels (1 for mono, 2 for stereo)

        Returns:
            Audio data as a string.

        Raises:
            RuntimeError: If synthesis fails.
        """
        request = speech_pb2.SynthesizeRequest(
            text=text,
            config=speech_pb2.AudioConfig(**kwargs) if kwargs else None,
        )

        response = await self.stub.Synthesize(request)
        if response.HasField("error"):
            raise RuntimeError(f"Synthesis error: {response.error}")
        return response.audio_data

    async def transcribe(self, audio_data: str) -> str:
        """Transcribe speech to text.

        Args:
            audio_data: Audio data to transcribe

        Returns:
            Transcribed text.

        Raises:
            RuntimeError: If transcription fails.
        """
        request = speech_pb2.TranscribeRequest(
            audio_data=audio_data,
        )

        response = await self.stub.Transcribe(request)
        if response.HasField("error"):
            raise RuntimeError(f"Transcription error: {response.error}")
        return response.text