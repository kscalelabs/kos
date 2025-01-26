"""Sim service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sim_pb2_grpc
from kos_protos.sim_pb2 import (
    DefaultPosition,
    GetParametersResponse,
    ResetRequest,
    SetParametersRequest,
    SetPausedRequest,
    SimulationParameters,
    StepRequest,
)


class DefaultPositionInput(TypedDict):
    """Initial simulation state.

    Fields:
        qpos: List of joint positions in simulation units
    """

    qpos: list[float]


class ResetParams(TypedDict):
    """Simulation reset parameters.

    Fields:
        initial_state: Optional DefaultPosition to set initial joint positions
        randomize: Optional flag to add randomization during reset
    """

    initial_state: NotRequired[DefaultPositionInput]
    randomize: NotRequired[bool]


class StepParams(TypedDict):
    """Parameters for stepping simulation.

    Fields:
        num_steps: Number of simulation steps to take
        step_size: Optional duration of each step in seconds
    """

    num_steps: int
    step_size: NotRequired[float]


class SimulationParams(TypedDict):
    """Parameters for configuring simulation.

    Fields:
        time_scale: Optional simulation time scale factor
        gravity: Optional gravitational acceleration in m/s²
        initial_state: Optional default joint positions
    """

    time_scale: NotRequired[float]
    gravity: NotRequired[float]
    initial_state: NotRequired[DefaultPositionInput]


class SimServiceClient:
    """Client for the simulation service.

    This client provides methods to control and configure the physics simulation,
    including resetting, stepping, pausing, and parameter adjustment.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the simulation service client.

        Args:
            channel: gRPC channel for communication with the service
        """
        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)

    def reset(self, **kwargs: Unpack[ResetParams]) -> common_pb2.ActionResponse:
        """Reset the simulation to a known state.

        Args:
            **kwargs: Reset parameters that may include:
                     initial_state: DefaultPosition to reset to
                     randomize: Whether to randomize the initial state

        Returns:
            ActionResponse indicating if the reset was successful
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = DefaultPosition(qpos=pos["qpos"])

        request = ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
        return self.stub.Reset(request)

    def set_paused(self, paused: bool) -> common_pb2.ActionResponse:
        """Pause or unpause the simulation.

        Args:
            paused: True to pause, False to unpause

        Returns:
            ActionResponse indicating if the pause state was set successfully
        """
        request = SetPausedRequest(paused=paused)
        return self.stub.SetPaused(request)

    def step(self, num_steps: int, step_size: float | None = None) -> common_pb2.ActionResponse:
        """Step the simulation forward by a specified number of steps.

        Args:
            num_steps: Number of simulation steps to take
            step_size: Optional duration of each step in seconds

        Returns:
            ActionResponse indicating if the stepping was successful
        """
        request = StepRequest(num_steps=num_steps, step_size=step_size)
        return self.stub.Step(request)

    def set_parameters(self, **kwargs: Unpack[SimulationParams]) -> common_pb2.ActionResponse:
        """Set simulation parameters.

        Args:
            **kwargs: Parameters that may include:
                     time_scale: Simulation time scale factor
                     gravity: Gravitational acceleration in m/s²
                     initial_state: Default joint positions

        Returns:
            ActionResponse indicating if the parameters were set successfully
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = DefaultPosition(qpos=pos["qpos"])

        params = SimulationParameters(
            time_scale=kwargs.get("time_scale"),
            gravity=kwargs.get("gravity"),
            initial_state=initial_state,
        )
        request = SetParametersRequest(parameters=params)
        return self.stub.SetParameters(request)

    def get_parameters(self) -> GetParametersResponse:
        """Get current simulation parameters.

        Returns:
            GetParametersResponse containing:
            - parameters: Current simulation parameters
            - error: Optional error information
        """
        return self.stub.GetParameters(Empty())
