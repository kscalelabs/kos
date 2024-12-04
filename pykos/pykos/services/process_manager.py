"""Process manager service client."""

from typing import Optional, Tuple

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import process_manager_pb2_grpc
from kos_protos.common_pb2 import Error
from kos_protos.process_manager_pb2 import KClipStartRequest


class ProcessManagerServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    def start_kclip(self, action: str) -> Tuple[Optional[str], Optional[Error]]:
        """Start KClip recording.

        Args:
            action: The action string for the KClip request

        Returns:
            Tuple containing:
            - clip_uuid (str): UUID of the started clip, if successful
            - error (Error): Error details if the operation failed
        """
        request = KClipStartRequest(action=action)
        response = self.stub.StartKClip(request)
        return response.clip_uuid, response.error if response.HasField("error") else None

    def stop_kclip(self, request: Empty = Empty()) -> Tuple[Optional[str], Optional[Error]]:
        """Stop KClip recording.

        Returns:
            Tuple containing:
            - clip_uuid (str): UUID of the stopped clip, if successful
            - error (Error): Error details if the operation failed
        """
        response = self.stub.StopKClip(request)
        return response.clip_uuid, response.error if response.HasField("error") else None
