"""Policy service client."""

import grpc.aio
from google.protobuf.empty_pb2 import Empty

from kos_protos import policy_pb2, policy_pb2_grpc
from kos_protos.policy_pb2 import StartPolicyRequest
from pykos.services import AsyncClientBase


class PolicyServiceClient(AsyncClientBase):
    """Client for the PolicyService."""

    def __init__(self, channel: grpc.aio.Channel) -> None:
        super().__init__()
        self.stub = policy_pb2_grpc.PolicyServiceStub(channel)

    async def start_policy(
        self, action: str, action_scale: float, episode_length: int, dry_run: bool
    ) -> policy_pb2.StartPolicyResponse:
        """Start policy execution.

        Args:
            action: The action string for the policy
            action_scale: Scale factor for actions
            episode_length: Length of the episode
            dry_run: Whether to perform a dry run

        Returns:
            The response from the server.
        """
        request = StartPolicyRequest(
            action=action,
            action_scale=action_scale,
            episode_length=episode_length,
            dry_run=dry_run,
        )
        return await self.stub.StartPolicy(request)

    async def stop_policy(self, request: Empty = Empty()) -> policy_pb2.StopPolicyResponse:
        """Stop policy execution.

        Returns:
            The response from the server.
        """
        return await self.stub.StopPolicy(request)

    async def get_state(self, request: Empty = Empty()) -> policy_pb2.GetStateResponse:
        """Get the current policy state.

        Returns:
            The response from the server containing the policy state.
        """
        return await self.stub.GetState(request)