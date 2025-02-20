"""Sim service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
import grpc.aio
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sim_pb2, sim_pb2_grpc
from pykos.services import AsyncClientBase


class StartingPosition(TypedDict):
    x: float
    y: float
    z: float


class StartingQuaternion(TypedDict):
    x: float
    y: float
    z: float
    w: float


class JointPosition(TypedDict):
    name: str
    pos: NotRequired[float]
    vel: NotRequired[float]


class ResetRequest(TypedDict):
    pos: NotRequired[StartingPosition]
    quat: NotRequired[StartingQuaternion]
    joints: NotRequired[list[JointPosition]]


class StepRequest(TypedDict):
    num_steps: int
    step_size: NotRequired[float]


class SimulationParameters(TypedDict):
    time_scale: NotRequired[float]
    gravity: NotRequired[float]


class SimServiceClient(AsyncClientBase):
    """Client for the SimulationService."""

    def __init__(self, channel: grpc.aio.Channel) -> None:
        super().__init__()

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
        pos = None
        if (pos_dict := kwargs.get("pos")) is not None:
            pos = sim_pb2.StartingPosition(
                x=pos_dict["x"],
                y=pos_dict["y"],
                z=pos_dict["z"],
            )

        quat = None
        if (quat_dict := kwargs.get("quat")) is not None:
            quat = sim_pb2.StartingQuaternion(
                x=quat_dict["x"],
                y=quat_dict["y"],
                z=quat_dict["z"],
                w=quat_dict["w"],
            )

        joints_values = None
        if (joints_dict := kwargs.get("joints")) is not None:
            joints_values = sim_pb2.JointValues(values=[sim_pb2.JointValue(**joint) for joint in joints_dict])

        request = sim_pb2.ResetRequest(pos=pos, quat=quat, joints=joints_values)
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
