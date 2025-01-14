"""Process manager service client."""

import grpc
from google.protobuf.empty_pb2 import Empty
from kos_protos import process_manager_pb2, process_manager_pb2_grpc
from kos_protos.process_manager_pb2 import KClipStartRequest


class ProcessManagerServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    def start_kclip(self, action: str) -> process_manager_pb2.KClipStartResponse:
        """Start KClip recording.

        Args:
            action: The action string for the KClip request

        Returns:
            The response from the server.
        """
        request = KClipStartRequest(action=action)
        return self.stub.StartKClip(request)

    def stop_kclip(self, request: Empty = Empty()) -> process_manager_pb2.KClipStopResponse:
        """Stop KClip recording.

        Returns:
            The response from the server.
        """
        return self.stub.StopKClip(request)
