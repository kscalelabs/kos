"""Process manager service client."""

import grpc.aio
from google.protobuf.empty_pb2 import Empty

from kos_protos import process_manager_pb2, process_manager_pb2_grpc
from kos_protos.process_manager_pb2 import KClipStartRequest
from pykos.services import AsyncClientBase


class ProcessManagerServiceClient(AsyncClientBase):
    """Client for the ProcessManagerService."""

    def __init__(self, channel: grpc.aio.Channel) -> None:
        super().__init__()

        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    async def start_kclip(self, action: str) -> process_manager_pb2.KClipStartResponse:
        """Start KClip recording.

        Args:
            action: The action string for the KClip request

        Returns:
            The response from the server.
        """
        request = KClipStartRequest(action=action)
        return await self.stub.StartKClip(request)

    async def stop_kclip(self, request: Empty = Empty()) -> process_manager_pb2.KClipStopResponse:
        """Stop KClip recording.

        Returns:
            The response from the server.
        """
        return await self.stub.StopKClip(request)
