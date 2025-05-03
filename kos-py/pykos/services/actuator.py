"""Actuator service client."""

import asyncio
import time
from typing import NotRequired, TypedDict, Unpack

import grpc
import grpc.aio
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos_protos import actuator_pb2, actuator_pb2_grpc, common_pb2
from kos_protos.actuator_pb2 import CalibrateActuatorMetadata
from pykos.services import AsyncClientBase


class ActuatorCommand(TypedDict):
    actuator_id: int
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]


class ActuatorPosition(TypedDict):
    actuator_id: int
    position: float


class ConfigureActuatorRequest(TypedDict):
    actuator_id: int
    kp: NotRequired[float]
    kd: NotRequired[float]
    ki: NotRequired[float]
    acceleration: NotRequired[float]
    max_torque: NotRequired[float]
    protective_torque: NotRequired[float]
    protection_time: NotRequired[float]
    torque_enabled: NotRequired[bool]
    new_actuator_id: NotRequired[int]
    zero_position: NotRequired[bool]


class ActuatorStateRequest(TypedDict):
    actuator_ids: list[int]


class CalibrationStatus:
    Calibrating = "calibrating"
    Calibrated = "calibrated"
    Timeout = "timeout"


class CalibrationMetadata:
    def __init__(self, metadata_any: AnyPb2) -> None:
        self.actuator_id: int | None = None
        self.status: str | None = None
        self.decode_metadata(metadata_any)

    def decode_metadata(self, metadata_any: AnyPb2) -> None:
        metadata = CalibrateActuatorMetadata()
        if metadata_any.Is(CalibrateActuatorMetadata.DESCRIPTOR):
            metadata_any.Unpack(metadata)
            self.actuator_id = metadata.actuator_id
            self.status = metadata.status if metadata.HasField("status") else None

    def __str__(self) -> str:
        return f"CalibrationMetadata(actuator_id={self.actuator_id}, status={self.status})"

    def __repr__(self) -> str:
        return self.__str__()


class ActuatorServiceClient(AsyncClientBase):
    """Client for the ActuatorService."""

    def __init__(self, channel: grpc.aio.Channel) -> None:
        super().__init__()
        self.stub = actuator_pb2_grpc.ActuatorServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    async def calibrate(self, actuator_id: int) -> CalibrationMetadata:
        """Calibrate an actuator."""
        response = await self.stub.CalibrateActuator(actuator_pb2.CalibrateActuatorRequest(actuator_id=actuator_id))
        metadata = CalibrationMetadata(response.metadata)
        return metadata

    async def get_calibration_status(self, actuator_id: int) -> str | None:
        """Get the calibration status of an actuator."""
        response = await self.operations_stub.GetOperation(
            operations_pb2.GetOperationRequest(name=f"operations/calibrate_actuator/{actuator_id}")
        )
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status

    async def command_actuators(self, commands: list[ActuatorCommand]) -> actuator_pb2.CommandActuatorsResponse:
        """Command multiple actuators at once.

        Example:
            >>> command_actuators([
            ...     {"actuator_id": 1, "position": 90.0, "velocity": 100.0, "torque": 1.0},
            ...     {"actuator_id": 2, "position": 180.0},
            ... ])

        Args:
            commands: List of dictionaries containing actuator commands.
                     Each dict should have 'actuator_id' and optionally 'position',
                     'velocity', and 'torque'.

        Returns:
            List of ActionResult objects indicating success/failure for each command.
        """
        actuator_commands = [actuator_pb2.ActuatorCommand(**cmd) for cmd in commands]
        request = actuator_pb2.CommandActuatorsRequest(commands=actuator_commands)
        return await self.stub.CommandActuators(request)

    async def configure_actuator(self, **kwargs: Unpack[ConfigureActuatorRequest]) -> common_pb2.ActionResult:
        """Configure an actuator's parameters.

        Example:
            >>> configure_actuator(
            ...     actuator_id=1,
            ...     kp=1.0,
            ...     kd=0.1,
            ...     ki=0.01,
            ...     acceleration=2230,
            ...     max_torque=100.0,
            ...     protective_torque=None,
            ...     protection_time=None,
            ...     torque_enabled=True,
            ...     new_actuator_id=None,
            ...     zero_position=True,
            ... )

            >>> configure_actuator(
            ...     actuator_id=2,
            ...     kp=1.0,
            ...     kd=0.1,
            ...     torque_enabled=True,
            ... )

        Args:
            actuator_id: ID of the actuator to configure
            **kwargs: Configuration parameters that may include:
                     kp, kd, ki, max_torque, protective_torque,
                     protection_time, torque_enabled, new_actuator_id

        Returns:
            ActionResponse indicating success/failure
        """
        request = actuator_pb2.ConfigureActuatorRequest(**kwargs)
        return await self.stub.ConfigureActuator(request)

    async def get_actuators_state(
        self,
        actuator_ids: list[int] | None = None,
    ) -> actuator_pb2.GetActuatorsStateResponse:
        """Get the state of multiple actuators.

        Example:
            >>> get_actuators_state([1, 2])

        Args:
            actuator_ids: List of actuator IDs to query. If None, gets state of all actuators.

        Returns:
            List of ActuatorStateResponse objects containing the state information
        """
        request = actuator_pb2.GetActuatorsStateRequest(actuator_ids=actuator_ids or [])
        return await self.stub.GetActuatorsState(request)

    async def move_to_position(
        self,
        positions: list[ActuatorPosition],
        num_seconds: float,
        configure_actuators: list[ConfigureActuatorRequest] | None = None,
        commands_per_second: int = 10,
        torque_enabled: bool | None = None,
    ) -> None:
        """Helper function for moving actuators to a target position.

        This first reads the current position of the actuators, then moves them
        to the target positions at a rate of `commands_per_second` commands per
        second.

        We can additionally use this command to safely configure the actuator
        PD parameters after setting the target position to the current position.

        Args:
            positions: The actuator target positions.
            num_seconds: How long to take the actuators to move to the target
                positions.
            configure_actuators: List of dictionaries containing actuator
                configuration parameters.
            commands_per_second: How many commands to send per second.
            torque_enabled: Whether to enable torque for the actuators.
        """
        actuator_ids = [p["actuator_id"] for p in positions]
        states = await self.get_actuators_state(actuator_ids)
        start_positions = {state.actuator_id: state.position for state in states.states}
        target_positions = {p["actuator_id"]: p["position"] for p in positions}

        if set(start_positions.keys()) != set(target_positions.keys()):
            raise ValueError("All actuator IDs must be present in both start and target positions")

        # Sets the target position to the current position for all actuators.
        await self.command_actuators(
            [
                {
                    "actuator_id": id,
                    "position": start_positions[id],
                    "velocity": 0.0,
                    "torque": 0.0,
                }
                for id in actuator_ids
            ]
        )

        # Computes target velocities for each actuator, in rad/s.
        velocities = {id: (target_positions[id] - start_positions[id]) / num_seconds for id in actuator_ids}

        # Optionally configure the actuators after setting the target position.
        if configure_actuators is not None:
            for configure_actuator in configure_actuators:
                if torque_enabled is not None:
                    configure_actuator["torque_enabled"] = torque_enabled
                await self.configure_actuator(**configure_actuator)
        elif torque_enabled is not None:
            for actuator_id in actuator_ids:
                await self.configure_actuator(
                    actuator_id=actuator_id,
                    torque_enabled=torque_enabled,
                )

        # Calculate the number of commands to send.
        num_commands = int(num_seconds * commands_per_second)

        # Send the commands.
        current_time = time.time()
        for i in range(num_commands):
            await self.command_actuators(
                [
                    {
                        "actuator_id": id,
                        "position": start_positions[id]
                        + (target_positions[id] - start_positions[id]) * (i / num_commands),
                        "velocity": velocities[id],
                    }
                    for id in actuator_ids
                ]
            )

            # Sleep until the next command is due.
            next_time = current_time + (1 / commands_per_second)
            if current_time < next_time:
                await asyncio.sleep(next_time - current_time)
            current_time = next_time

        # Finally, set the velocity to 0 for all actuators.
        await self.command_actuators(
            [
                {
                    "actuator_id": id,
                    "position": target_positions[id],
                    "velocity": 0.0,
                }
                for id in actuator_ids
            ]
        )

    async def zero_actuators(
        self,
        actuator_id: int,
        zero_position: float,
        configure_actuator: ConfigureActuatorRequest | None = None,
        target_velocity: float = 0.25,
        commands_per_second: int = 10,
        move_back_seconds: float = 3.0,
    ) -> None:
        """Helper method for zeroing an actuator.

        This function works to zero the actuator against an endstop, by moving
        the actuator until it reaches that endstop, then moving it back a small
        amount.

        We can choose which endstop to zero against by setting the sign of
        `zero_position`. If it is positive, then we rotate counterclockwise
        until we reach an endstop, then back by this amount. If it is negative,
        then we rotate clockwise until we reach an endstop, then back by this
        amount.

        Args:
            actuator_id: The ID of the actuator to zero.
            zero_position: The position to move the actuator back by after
                reaching the endstop.
            configure_actuator: Configuration parameters to set on the actuator
                before zeroing.
            target_velocity: The velocity to move the actuator at.
            commands_per_second: How many commands to send per second.
            move_back_seconds: How long to move the actuator back by after
                reaching the endstop.
        """
        start_state = await self.get_actuators_state([actuator_id])
        current_position = start_state.states[0].position

        # Sets the target position to the current position for all actuators.
        await self.command_actuators(
            [
                {
                    "actuator_id": actuator_id,
                    "position": current_position,
                    "velocity": 0.0,
                    "torque": 0.0,
                }
            ]
        )

        if configure_actuator is not None:
            configure_actuator["torque_enabled"] = True
            await self.configure_actuator(**configure_actuator)
        else:
            await self.configure_actuator(actuator_id=actuator_id, torque_enabled=True)

        if target_velocity <= 0:
            raise ValueError("target_velocity must be positive")

        # Calculates the position delta.
        delta = target_velocity / commands_per_second

        # Ensure the target velocity is oriented correctly.
        if zero_position > 0:
            target_velocity = -target_velocity
            delta = -delta
        elif zero_position == 0:
            raise ValueError("zero_position must be non-zero")

        current_time = time.time()
        while True:
            current_state = await self.get_actuators_state([actuator_id])
            current_read_position = current_state.states[0].position

            # If we are not able to reach the target position, it means we are
            # hitting against the endstop, so we should break.
            if abs(current_read_position - current_position) >= abs(delta / 2):
                break

            # Adds the delta to the current position.
            current_position = current_position + delta

            await self.command_actuators(
                [
                    {
                        "actuator_id": actuator_id,
                        "position": current_position,
                        "velocity": target_velocity,
                    }
                ]
            )

            # Sleep until the next command is due.
            next_time = current_time + (1 / commands_per_second)
            if current_time < next_time:
                await asyncio.sleep(next_time - current_time)
            current_time = next_time

        # Move back by the zero position.
        await self.move_to_position(
            positions=[
                {
                    "actuator_id": actuator_id,
                    "position": current_position - zero_position,
                },
            ],
            num_seconds=move_back_seconds,
        )

        # Finally, configure the new location as the actuator zero.
        await self.configure_actuator(
            actuator_id=actuator_id,
            zero_position=True,
        )

    async def parameter_dump(
        self,
        actuator_ids: list[int],
    ) -> actuator_pb2.ParameterDumpResponse:
        """Fetch parameters for specified actuators.

        Args:
            actuator_ids: List of actuator IDs to query.

        Returns:
            A ParameterDumpResponse containing parameter maps for each actuator.
        """
        request = actuator_pb2.ParameterDumpRequest(actuator_ids=actuator_ids)
        return await self.stub.ParameterDump(request)
