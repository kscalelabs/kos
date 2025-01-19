"""Actuator service client."""

from typing import List, NotRequired, TypedDict, Unpack

import grpc
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos_protos import actuator_pb2, actuator_pb2_grpc, common_pb2
from kos_protos.actuator_pb2 import CalibrateActuatorMetadata


class ActionResult(TypedDict):
    actuator_id: int
    success: bool
    error: NotRequired[common_pb2.Error]


class CommandActuatorsResponse(TypedDict):
    results: List[ActionResult]


class ActuatorStateResponse(TypedDict):
    actuator_id: int
    online: bool
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]
    temperature: NotRequired[float]
    voltage: NotRequired[float]
    current: NotRequired[float]


class GetActuatorsStateResponse(TypedDict):
    states: List[ActuatorStateResponse]


class ActuatorCommand(TypedDict):
    actuator_id: int
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]


class ConfigureActuatorRequest(TypedDict):
    actuator_id: int
    kp: NotRequired[float]
    kd: NotRequired[float]
    ki: NotRequired[float]
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


class ActuatorServiceClient:
    def __init__(self, channel: grpc.Channel) -> None:
        self.stub = actuator_pb2_grpc.ActuatorServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    def calibrate(self, actuator_id: int) -> CalibrationMetadata:
        """Calibrate an actuator.

        Returns:
            Operation: The operation for the calibration.
        """
        response = self.stub.CalibrateActuator(actuator_pb2.CalibrateActuatorRequest(actuator_id=actuator_id))
        metadata = CalibrationMetadata(response.metadata)
        return metadata

    def get_calibration_status(self, actuator_id: int) -> str | None:
        response = self.operations_stub.GetOperation(
            operations_pb2.GetOperationRequest(name=f"operations/calibrate_actuator/{actuator_id}")
        )
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status

    def command_actuators(self, commands: list[ActuatorCommand]) -> CommandActuatorsResponse:
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
            CommandActuatorsResponse is a list of ActionResult objects indicating success/failure for each command.
        """
        actuator_commands = [actuator_pb2.ActuatorCommand(**cmd) for cmd in commands]
        request = actuator_pb2.CommandActuatorsRequest(commands=actuator_commands)
        return self.stub.CommandActuators(request)

    def configure_actuator(self, **kwargs: Unpack[ConfigureActuatorRequest]) -> ActionResult:
        """Configure an actuator's parameters.

        Example:
            >>> configure_actuator(
            ...     actuator_id=1,
            ...     kp=1.0,
            ...     kd=0.1,
            ...     ki=0.01,
            ...     max_torque=100.0,
            ...     protective_torque=None,
            ...     protection_time=None,
            ...     torque_enabled=True,
            ...     new_actuator_id=None,
            ...     zero_position=True
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
            ActionResult conatining the actuator_id and a ActionResponse indicating success/failure
        """
        request = actuator_pb2.ConfigureActuatorRequest(**kwargs)
        return self.stub.ConfigureActuator(request)

    def get_actuators_state(self, actuator_ids: list[int] | None = None) -> GetActuatorsStateResponse:
        """Get the state of multiple actuators.

        Example:
            >>> get_actuators_state([1, 2])

        Args:
            actuator_ids: List of actuator IDs to query. If None, gets state of all actuators.

        Returns:
            GetActuatorsStateResponse containing a list of ActuatorStateResponse objects with state information
        """
        request = actuator_pb2.GetActuatorsStateRequest(actuator_ids=actuator_ids or [])
        return self.stub.GetActuatorsState(request)
