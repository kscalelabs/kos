"""Sim service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
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
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)

    def reset(self, **kwargs: Unpack[ResetRequest]) -> common_pb2.ActionResponse:
        """Reset the simulation to its initial state.

        Example:
            reset(initial_state={"qpos": [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]})
            reset(randomize=True)

        Args:
            **kwargs: Reset parameters that may include:
                     initial_state: DefaultPosition to reset to
                     randomize: Whether to randomize the initial state

        Returns:
            ActionResponse indicating success/failure
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])

        request = sim_pb2.ResetRequest(
            initial_state=initial_state,
            randomize=kwargs.get("randomize")
        )
        return self.stub.Reset(request)

    def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
        """Pause or unpause the simulation.

        Args:
            paused: True to pause, False to unpause

        Returns:
            ActionResponse indicating success/failure
        """
        request = sim_pb2.SetPausedRequest(paused=paused)
        return self.stub.SetPaused(request)

    def step(self, num_steps: int = 1, step_size: float | None = None) -> common_pb2.ActionResponse:
        """Step the simulation forward.

        Args:
            num_steps: Number of simulation steps to take
            step_size: Optional time per step in seconds

        Returns:
            ActionResponse indicating success/failure
        """
        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
        return self.stub.Step(request)

    def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> common_pb2.ActionResponse:
        """Set simulation parameters.

        Example:
            set_parameters(time_scale=1.0, gravity=9.81, initial_state={"qpos": [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]})

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
            time_scale=kwargs.get("time_scale"),
            gravity=kwargs.get("gravity"),
            initial_state=initial_state
        )
        request = sim_pb2.SetParametersRequest(parameters=params)
        return self.stub.SetParameters(request)

    def get_parameters(self) -> sim_pb2.GetParametersResponse:
        """Get current simulation parameters.

        Returns:
            GetParametersResponse containing current parameters and any error
        """
        return self.stub.GetParameters(Empty())

