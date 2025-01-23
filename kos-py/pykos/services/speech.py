"""Speech service client."""

import grpc
import grpc.aio

from kos_protos import speech_pb2, speech_pb2_grpc


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

    async def synthesize(self, text: str) -> speech_pb2.SynthesizeResponse:
        """Synthesize speech from text.

        Args:
            text: Text to synthesize

        Returns:
            Audio data as a string.

        Raises:
            RuntimeError: If synthesis fails.
        """
        request = speech_pb2.SynthesizeRequest(text=text)

        response = await self.stub.Synthesize(request)
        if response.HasField("error"):
            raise RuntimeError(f"Synthesis error: {response.error}")
        return response.file_path

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
