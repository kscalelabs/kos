"""Process manager service client."""

from typing import NotRequired, TypedDict

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, process_manager_pb2_grpc
from kos_protos.process_manager_pb2 import KClipStartRequest


class KClipStartResponse(TypedDict):
    """TypedDict containing response from starting a KClip recording.

    A dictionary type containing information about a newly started
    KClip recording session, including its unique identifier.

    Fields:
        success: Whether the recording started successfully
        clip_uuid: Unique identifier for the recording session
        error: Optional error information if start failed
    """

    success: bool
    clip_uuid: str
    error: NotRequired[common_pb2.Error | None]


class KClipStopResponse(TypedDict):
    """TypedDict containing response from stopping a KClip recording.

    A dictionary type containing information about the stopped
    KClip recording session, including its identifier.

    Fields:
        success: Whether the recording stopped successfully
        clip_uuid: Identifier of the stopped recording session
        error: Optional error information if stop failed
    """

    success: bool
    clip_uuid: str
    error: NotRequired[common_pb2.Error | None]


class ProcessManagerServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    def start_kclip(self, action: str) -> KClipStartResponse:
        """Start KClip recording.

        Args:
            action: The action string for the KClip request

        Returns:
            KClipStartResponse is a dictionary where:
            - 'success' indicates if the recording started successfully
            - 'clip_uuid' contains the unique identifier for the recording session
            - 'error' contains any error information if the start failed
        """
        request = KClipStartRequest(action=action)
        return self.stub.StartKClip(request)

    def stop_kclip(self, request: Empty = Empty()) -> KClipStopResponse:
        """Stop KClip recording.

        Returns:
            KClipStopResponse is a dictionary where:
            - 'success' indicates if the recording stopped successfully
            - 'clip_uuid' contains the identifier of the stopped recording session
            - 'error' contains any error information if the stop failed
        """
        return self.stub.StopKClip(request)
