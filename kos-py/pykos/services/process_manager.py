"""Process manager service client."""

from typing import NotRequired, TypedDict

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, process_manager_pb2_grpc
from kos_protos.process_manager_pb2 import KClipStartRequest


class KClipStartResponse(TypedDict):
    """Response from starting KClip recording."""

    success: bool
    clip_uuid: str
    error: NotRequired[common_pb2.Error | None]


class KClipStopResponse(TypedDict):
    """Response from stopping KClip recording."""

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
            KClipStartResponse containing success status and any error information.
        """
        request = KClipStartRequest(action=action)
        return self.stub.StartKClip(request)

    def stop_kclip(self, request: Empty = Empty()) -> KClipStopResponse:
        """Stop KClip recording.

        Returns:
            KClipStopResponse containing success status, clip path, and any error information.
        """
        return self.stub.StopKClip(request)
