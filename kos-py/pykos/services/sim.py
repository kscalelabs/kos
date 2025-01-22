"""Sim service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
import grpc.aio
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sim_pb2, sim_pb2_grpc


class DefaultPosition(TypedDict):
    qpos: list[float]


class ResetRequest(TypedDict):
    initial_state: NotRequired[DefaultPosition]
    randomize: NotRequired[bool]


class StepRequest(TypedDict):
    num_steps: int
    step_size: NotRequired[float]


class SimulationParameters(TypedDict):
    time_scale: NotRequired[float]
    gravity: NotRequired[float]
    initial_state: NotRequired[DefaultPosition]


class SimServiceClient:
    def __init__(self, channel: grpc.aio.Channel) -> None:
        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)

    async def reset(self, **kwargs: Unpack[ResetRequest]) -> common_pb2.ActionResponse:
        """Reset the simulation to its initial state.

        Args:
            **kwargs: Reset parameters that may include:
                     initial_state: DefaultPosition to reset to
                     randomize: Whether to randomize the initial state

        Example:
            >>> client.reset(
            ...     initial_state={"qpos": [0.0, 0.0, 0.0]},
            ...     randomize=True
            ... )

        Returns:
            ActionResponse indicating success/failure
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])

        request = sim_pb2.ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
        return await self.stub.Reset(request)

    async def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
        """Pause or unpause the simulation.

        Args:
            paused: True to pause, False to unpause

        Returns:
            ActionResponse indicating success/failure
        """
        request = sim_pb2.SetPausedRequest(paused=paused)
        return await self.stub.SetPaused(request)

    async def step(self, num_steps: int, step_size: float | None = None) -> common_pb2.ActionResponse:
        """Step the simulation forward.

        Args:
            num_steps: Number of simulation steps to take
            step_size: Optional time per step in seconds

        Returns:
            ActionResponse indicating success/failure
        """
        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
        return await self.stub.Step(request)

    async def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> common_pb2.ActionResponse:
        """Set simulation parameters.

        Example:
        >>> client.set_parameters(
        ...     time_scale=1.0,
        ...     gravity=9.81,
        ...     initial_state={"qpos": [0.0, 0.0, 0.0]}
        ... )

        Args:
            **kwargs: Parameters that may include:
                     time_scale: Simulation time scale
                     gravity: Gravity constant
                     initial_state: Default position state

        Returns:
            ActionResponse indicating success/failure
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])

        params = sim_pb2.SimulationParameters(
            time_scale=kwargs.get("time_scale"), gravity=kwargs.get("gravity"), initial_state=initial_state
        )
        request = sim_pb2.SetParametersRequest(parameters=params)
        return await self.stub.SetParameters(request)

    async def get_parameters(self) -> sim_pb2.GetParametersResponse:
        """Get current simulation parameters.

        Returns:
            GetParametersResponse containing current parameters and any error
        """
        return await self.stub.GetParameters(Empty())
