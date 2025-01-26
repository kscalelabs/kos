"""Actuator service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos_protos import actuator_pb2, actuator_pb2_grpc, common_pb2
from kos_protos.actuator_pb2 import (
    CalibrateActuatorMetadata,
    CommandActuatorsRequest,
    CommandActuatorsResponse,
    ConfigureActuatorRequest,
    GetActuatorsStateRequest,
    GetActuatorsStateResponse,
)


class ActuatorCommand(TypedDict):
    """Command parameters for an actuator.

    Fields:
        actuator_id: The ID of the actuator to command
        position: Optional target position in degrees
        velocity: Optional target velocity in degrees/second
        torque: Optional target torque in Nm
    """

    actuator_id: int
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]


class ConfigureActuatorParams(TypedDict):
    """Configuration parameters for an actuator.

    Fields:
        actuator_id: The ID of the actuator to configure
        kp: Optional proportional gain for position control
        kd: Optional derivative gain for position control
        ki: Optional integral gain for position control
        max_torque: Optional maximum torque limit
        protective_torque: Optional protective torque threshold
        protection_time: Optional protection activation time
        torque_enabled: Optional flag to enable/disable torque
        new_actuator_id: Optional new ID to assign to the actuator
        zero_position: Optional flag to set current position as zero
    """

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
            if metadata.HasField("actuator_id"):
                self.actuator_id = metadata.actuator_id
            if metadata.HasField("status"):
                self.status = metadata.status

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

        Args:
            actuator_id: The ID of the actuator to calibrate

        Returns:
            CalibrationMetadata object containing the actuator ID and calibration status
        """
        request = actuator_pb2.CalibrateActuatorRequest(actuator_id=actuator_id)
        operation = self.stub.CalibrateActuator(request)
        return CalibrationMetadata(operation.metadata)

    def get_calibration_status(self, actuator_id: int) -> str | None:
        """Get the calibration status of an actuator.

        Args:
            actuator_id: The ID of the actuator to check

        Returns:
            The calibration status string or None if not found
        """
        request = operations_pb2.GetOperationRequest(name=str(actuator_id))
        operation = self.operations_stub.GetOperation(request)
        return CalibrationMetadata(operation.metadata).status if operation.metadata else None

    def command_actuators(self, commands: list[ActuatorCommand]) -> CommandActuatorsResponse:
        """Send commands to multiple actuators.

        Args:
            commands: List of dictionaries specifying commands for each actuator.
                     Each dictionary must have 'actuator_id' and may include:
                     - position: Target position in degrees
                     - velocity: Target velocity in degrees/second
                     - torque: Target torque in Nm

        Returns:
            CommandActuatorsResponse containing results for each command
        """
        proto_commands = [actuator_pb2.ActuatorCommand(**cmd) for cmd in commands]
        request = CommandActuatorsRequest(commands=proto_commands)
        return self.stub.CommandActuators(request)

    def configure_actuator(self, **kwargs: Unpack[ConfigureActuatorParams]) -> common_pb2.ActionResponse:
        """Configure an actuator with the specified parameters.

        Example:
            >>> configure_actuator(
            ...     actuator_id=1,
            ...     kp=10.0,
            ...     kd=1.0,
            ...     ki=0.1,
            ...     max_torque=2.0,
            ...     protective_torque=1.5,
            ...     protection_time=0.5,
            ...     torque_enabled=True,
            ...     new_actuator_id=2,
            ...     zero_position=True
            ... )

        Args:
            **kwargs: Configuration parameters that must include:
                     actuator_id: The ID of the actuator to configure
                     And may optionally include:
                     kp: Proportional gain for position control
                     kd: Derivative gain for position control
                     ki: Integral gain for position control
                     max_torque: Maximum torque limit
                     protective_torque: Protective torque threshold
                     protection_time: Protection activation time
                     torque_enabled: Flag to enable/disable torque
                     new_actuator_id: New ID to assign to the actuator
                     zero_position: Flag to set current position as zero

        Returns:
            ActionResponse indicating if the configuration was successful
        """
        request = ConfigureActuatorRequest(**kwargs)
        return self.stub.ConfigureActuator(request)

    def get_actuators_state(self, actuator_ids: list[int] | None = None) -> GetActuatorsStateResponse:
        """Get the current state of specified actuators.

        Args:
            actuator_ids: Optional list of actuator IDs to query. If None, queries all actuators.

        Returns:
            GetActuatorsStateResponse containing states for each queried actuator
        """
        request = GetActuatorsStateRequest(actuator_ids=actuator_ids or [])
        return self.stub.GetActuatorsState(request)
