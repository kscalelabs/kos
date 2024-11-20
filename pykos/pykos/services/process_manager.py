"Process manager service client"

import grpc
from google.protobuf.empty_pb2 import Empty
from typing import Optional, Tuple

from kos import process_manager_pb2, process_manager_pb2_grpc
from kos.common_pb2 import Error


class ProcessManagerServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = process_manager_pb2_grpc.ProcessManagerServiceStub(channel)

    def start_kclip(self) -> Tuple[Optional[str], Optional[Error]]:
        """Start KClip recording.

        Returns:
            Tuple containing:
            - clip_uuid (str): UUID of the started clip, if successful
            - error (Error): Error details if the operation failed
        """
        response = self.stub.StartKClip(Empty())
        return response.clip_uuid, response.error if response.HasField("error") else None

    def stop_kclip(self) -> Tuple[Optional[str], Optional[Error]]:
        """Stop KClip recording.

        Returns:
            Tuple containing:
            - clip_uuid (str): UUID of the stopped clip, if successful
            - error (Error): Error details if the operation failed
        """
        response = self.stub.StopKClip(Empty())
        return response.clip_uuid, response.error if response.HasField("error") else None

