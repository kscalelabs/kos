"""Actuator service client."""

from typing import NotRequired, TypedDict, Unpack

import grpc
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos_protos import actuator_pb2, actuator_pb2_grpc, common_pb2
from kos_protos.actuator_pb2 import CalibrateActuatorMetadata


class ActionResult(TypedDict):
    """TypedDict containing the result of an actuator action.

    A dictionary type that includes the actuator ID and whether the action succeeded.

    Fields:
        actuator_id: The ID of the actuator that performed the action
        success: Whether the action completed successfully
        error: Optional error information if the action failed
    """

    actuator_id: int
    success: bool
    error: NotRequired[common_pb2.Error | None]


class CommandActuatorsResponse(TypedDict):
    """TypedDict containing response from actuator command execution.

    A dictionary type containing a list of ActionResult objects for each command sent.

    Fields:
        results: List of ActionResult objects indicating success/failure for each command
        error: Optional error information if the overall command failed
    """

    results: list[ActionResult]
    error: NotRequired[common_pb2.Error | None]


class ActuatorStateResponse(TypedDict):
    """TypedDict containing the current state of an actuator.

    A dictionary type containing various measurements and states for a specific actuator.

    Fields:
        actuator_id: The ID of the actuator
        online: Whether the actuator is currently online
        position: Optional current position of the actuator
        velocity: Optional current velocity of the actuator
        torque: Optional current torque of the actuator
        temperature: Optional current temperature of the actuator
        voltage: Optional current voltage of the actuator
        current: Optional current draw of the actuator
    """

    actuator_id: int
    online: bool
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]
    temperature: NotRequired[float]
    voltage: NotRequired[float]
    current: NotRequired[float]


class GetActuatorsStateResponse(TypedDict):
    """TypedDict containing response for actuator state query.

    A dictionary type containing a list of actuator states.

    Fields:
        states: List of ActuatorStateResponse objects for each queried actuator
        error: Optional error information if the query failed
    """

    states: list[ActuatorStateResponse]
    error: NotRequired[common_pb2.Error | None]


class ActuatorCommand(TypedDict):
    """TypedDict containing command parameters for an actuator.

    A dictionary type specifying various control parameters for an actuator.

    Fields:
        actuator_id: The ID of the actuator to command
        position: Optional target position to move to
        velocity: Optional target velocity to maintain
        torque: Optional target torque to apply
    """

    actuator_id: int
    position: NotRequired[float]
    velocity: NotRequired[float]
    torque: NotRequired[float]


class ConfigureActuatorRequest(TypedDict):
    """TypedDict containing configuration parameters for an actuator.

    A dictionary type specifying various configuration options for an actuator.

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
            CommandActuatorsResponse is a dictionary where the key 'results' corresponds to a list of
            ActionResult objects indicating success/failure for each command.
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
            ActionResult is a dictionary containing the actuator_id and a success flag indicating
            whether the configuration was successful.
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
            GetActuatorsStateResponse is a dictionary where the key 'states' corresponds to a list of
            ActuatorStateResponse objects containing the current state of each queried actuator.
        """
        request = actuator_pb2.GetActuatorsStateRequest(actuator_ids=actuator_ids or [])
        return self.stub.GetActuatorsState(request)
