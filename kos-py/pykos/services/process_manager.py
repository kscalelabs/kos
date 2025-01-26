"""Process manager service client."""

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import process_manager_pb2_grpc
from kos_protos.process_manager_pb2 import (
    KClipStartRequest,
    KClipStartResponse,
    KClipStopResponse,
)


class ProcessManagerServiceClient:
    """Client for the process manager service.

    This client provides methods to manage KClip recordings and other processes.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the process manager service client.

        Args:
            channel: gRPC channel for communication with the service
        """
        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    def start_kclip(self, action: str) -> KClipStartResponse:
        """Start a new KClip recording.

        Args:
            action: The action being recorded

        Returns:
            KClipStartResponse containing:
            - clip_uuid: Unique identifier for the recording session
            - error: Optional error information if start failed
        """
        request = KClipStartRequest(action=action)
        return self.stub.StartKClip(request)

    def stop_kclip(self, request: Empty = Empty()) -> KClipStopResponse:
        """Stop the current KClip recording.

        Returns:
            KClipStopResponse containing:
            - clip_uuid: Identifier of the stopped recording session
            - error: Optional error information if stop failed
        """
        return self.stub.StopKClip(request)
