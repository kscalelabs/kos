"""Sim service client."""

from typing import Literal, NotRequired, TypedDict, Unpack

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


class MarkerRGBA(TypedDict):
    r: float
    g: float
    b: float
    a: float


class Marker(TypedDict):
    name: str
    marker_type: Literal["sphere", "box", "capsule", "cylinder", "arrow"]
    target_name: str
    target_type: Literal["geom", "body"]
    scale: list[float]
    offset: NotRequired[list[float]]
    color: NotRequired[MarkerRGBA]
    label: NotRequired[bool]
    track_rotation: NotRequired[bool]


class UpdateMarkerRequest(TypedDict):
    name: str
    marker_type: NotRequired[Literal["sphere", "box", "capsule", "cylinder", "arrow"]]
    offset: NotRequired[list[float]]
    color: NotRequired[MarkerRGBA]
    scale: NotRequired[list[float]]
    label: NotRequired[bool]


class SimServiceClient(AsyncClientBase):
    """Client for the SimulationService."""

    def __init__(self, channel: grpc.aio.Channel) -> None:
        super().__init__()

        self.stub = sim_pb2_grpc.SimulationServiceStub(channel)

    async def add_marker(self, **kwargs: Unpack[Marker]) -> common_pb2.ActionResponse:
        """Add a marker to the simulation.

        Args:
            **kwargs: Marker parameters
            name: Name of the marker
            type: Type of marker
            track_name: Name of the track to attach the marker to
            track_type: Type of track to attach the marker to
            scale: Scale of the marker
            offset: (optional) Offset of the marker
            color: (optional) Color of the marker
            label: (optional) Whether to show the marker label
            track_rotation: (optional) Whether to track the marker rotation

        Returns:
            ActionResponse indicating success/failure
        """
        if offset_list := kwargs.get("offset"):
            offset = sim_pb2.Marker.Offset(x=offset_list[0], y=offset_list[1], z=offset_list[2])
        else:
            offset = sim_pb2.Marker.Offset(x=0.0, y=0.0, z=0.0)

        if color_dataclass := kwargs.get("color"):
            color = sim_pb2.Marker.RGBA(
                r=color_dataclass["r"], g=color_dataclass["g"], b=color_dataclass["b"], a=color_dataclass["a"]
            )
        else:
            color = sim_pb2.Marker.RGBA(r=1.0, g=0.0, b=0.0, a=1.0)
        request = sim_pb2.Marker(
            name=kwargs["name"],
            marker_type=getattr(sim_pb2.Marker.MarkerType, str.upper(kwargs["marker_type"])),
            target_name=kwargs["target_name"],
            target_type=getattr(sim_pb2.Marker.TargetType, str.upper(kwargs["target_type"])),
            scale=sim_pb2.Marker.Scale(scale=kwargs["scale"]),
            color=color,
            label=kwargs.get("label", False),
            track_rotation=kwargs.get("track_rotation", False),
            offset=offset,
        )

        return await self.stub.AddMarker(request)

    async def update_marker(self, name: str, **kwargs: Unpack[UpdateMarkerRequest]) -> common_pb2.ActionResponse:
        """Update a marker in the simulation.

        Args:
            name: Name of the marker to update
            **kwargs: Marker parameters
            type: (optional) Type of marker
            offset: (optional) Offset of the marker
            color: (optional) Color of the marker
            label: (optional) Whether to show the marker label
            track_rotation: (optional) Whether to track the marker rotation

        Returns:
            ActionResponse indicating success/failure
        """
        if type_str := kwargs.get("marker_type"):
            marker_type = getattr(sim_pb2.Marker.MarkerType, str.upper(type_str))
        else:
            marker_type = None

        if offset_list := kwargs.get("offset"):
            offset = sim_pb2.Marker.Offset(x=offset_list[0], y=offset_list[1], z=offset_list[2])
        else:
            offset = None

        if color_dict := kwargs.get("color"):
            color = sim_pb2.Marker.RGBA(r=color_dict["r"], g=color_dict["g"], b=color_dict["b"], a=color_dict["a"])
        else:
            color = None

        if scale_list := kwargs.get("scale"):
            scale = sim_pb2.Marker.Scale(scale=scale_list)
        else:
            scale = None

        request = sim_pb2.UpdateMarkerRequest(
            name=name,
            marker_type=marker_type,
            offset=offset,
            color=color,
            label=kwargs.get("label"),
            scale=scale,
        )

        return await self.stub.UpdateMarker(request)

    async def remove_marker(self, name: str) -> common_pb2.ActionResponse:
        """Remove a marker from the simulation.

        Args:
            name: Name of the marker to remove

        Returns:
            ActionResponse indicating success/failure
        """
        request = sim_pb2.RemoveMarkerRequest(name=name)
        return await self.stub.RemoveMarker(request)

    async def get_markers(self) -> sim_pb2.GetMarkersResponse:
        """Get all markers in the simulation.

        Returns:
            GetMarkersResponse containing all markers
        """
        return await self.stub.GetMarkers(Empty())

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
        ... )

        Args:
            **kwargs: Parameters that may include:
                     time_scale: Simulation time scale
                     gravity: Gravity constant
                     initial_state: Default position state

        Returns:
            ActionResponse indicating success/failure
        """
        params = sim_pb2.SimulationParameters(
            time_scale=kwargs.get("time_scale"),
            gravity=kwargs.get("gravity"),
        )
        request = sim_pb2.SetParametersRequest(parameters=params)
        return await self.stub.SetParameters(request)

    async def get_parameters(self) -> sim_pb2.GetParametersResponse:
        """Get current simulation parameters.

        Returns:
            GetParametersResponse containing current parameters and any error
        """
        return await self.stub.GetParameters(Empty())
