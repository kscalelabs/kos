"""Sim service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.protobuf.empty_pb2 import Empty

from kos_protos import common_pb2, sim_pb2, sim_pb2_grpc


class DefaultPosition(TypedDict):
    """TypedDict containing initial simulation state.

    A dictionary type specifying the initial joint positions for
    resetting the simulation to a known state.

    Fields:
        qpos: List of joint positions in simulation units
    """

    qpos: list[float]


class ResetRequest(TypedDict):
    """TypedDict containing simulation reset parameters.

    A dictionary type specifying how the simulation should be reset,
    including optional initial state and randomization settings.

    Fields:
        initial_state: Optional DefaultPosition to set initial joint positions
        randomize: Optional flag to add randomization during reset
    """

    initial_state: NotRequired[DefaultPosition]
    randomize: NotRequired[bool]


class StepRequest(TypedDict):
    """Request parameters for stepping simulation."""

    num_steps: int
    step_size: NotRequired[float]


class SimulationParameters(TypedDict):
    """Parameters for configuring simulation."""

    time_scale: NotRequired[float]
    gravity: NotRequired[float]
    initial_state: NotRequired[DefaultPosition]


class ActionResponse(TypedDict):
    """Response indicating success/failure of an action."""

    success: bool
    error: NotRequired[common_pb2.Error | None]


class GetParametersResponse(TypedDict):
    """Response containing current simulation parameters."""

    time_scale: float
    gravity: float
    initial_state: DefaultPosition
    error: NotRequired[common_pb2.Error | None]


class SimServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)

    def reset(self, **kwargs: Unpack[ResetRequest]) -> ActionResponse:
        """Reset the simulation to its initial state.

        Example:
            >>> reset(
            ...     initial_state={"qpos": [0.0, 0.0, 0.0]},
            ...     randomize=True
            ... )

        Args:
            **kwargs: Reset parameters that may include:
                     initial_state: DefaultPosition to reset to
                     randomize: Whether to randomize the initial state

        Returns:
            ActionResponse is a dictionary where 'success' indicates if the reset operation
            was successful, and 'error' contains any error information if the reset failed.
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])

        request = sim_pb2.ResetRequest(initial_state=initial_state, randomize=kwargs.get("randomize"))
        return self.stub.Reset(request)

    def set_paused(self, paused: bool) -> ActionResponse:
        """Pause or unpause the simulation.

        Example:
            >>> set_paused(True)  # Pause simulation
            >>> set_paused(False)  # Resume simulation

        Args:
            paused: True to pause, False to unpause

        Returns:
            ActionResponse is a dictionary where 'success' indicates if the pause state
            was set successfully, and 'error' contains any error information if the operation failed.
        """
        request = sim_pb2.SetPausedRequest(paused=paused)
        return self.stub.SetPaused(request)

    def step(self, num_steps: int, step_size: float | None = None) -> ActionResponse:
        """Step the simulation forward.

        Example:
            >>> step(num_steps=100, step_size=0.001)  # Step forward 100 times with 1ms steps
            >>> step(num_steps=50)  # Step forward 50 times with default step size

        Args:
            num_steps: Number of simulation steps to take
            step_size: Optional time per step in seconds

        Returns:
            ActionResponse is a dictionary where 'success' indicates if the stepping operation
            was successful, and 'error' contains any error information if the stepping failed.
        """
        request = sim_pb2.StepRequest(num_steps=num_steps, step_size=step_size)
        return self.stub.Step(request)

    def set_parameters(self, **kwargs: Unpack[SimulationParameters]) -> ActionResponse:
        """Set simulation parameters.

        Example:
            >>> set_parameters(
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
            ActionResponse is a dictionary where 'success' indicates if the parameters were
            set successfully, and 'error' contains any error information if the operation failed.
        """
        initial_state = None
        if "initial_state" in kwargs:
            pos = kwargs["initial_state"]
            initial_state = sim_pb2.DefaultPosition(qpos=pos["qpos"])

        params = sim_pb2.SimulationParameters(
            time_scale=kwargs.get("time_scale"), gravity=kwargs.get("gravity"), initial_state=initial_state
        )
        request = sim_pb2.SetParametersRequest(parameters=params)
        return self.stub.SetParameters(request)

    def get_parameters(self) -> GetParametersResponse:
        """Get current simulation parameters.

        Example:
            >>> get_parameters()

        Returns:
            GetParametersResponse is a dictionary where:
            - 'time_scale' contains the current simulation time scaling factor
            - 'gravity' contains the current gravity constant value
            - 'initial_state' contains the default position state as a DefaultPosition dictionary
            - 'error' contains any error information if the query failed
        """
        return self.stub.GetParameters(Empty())
