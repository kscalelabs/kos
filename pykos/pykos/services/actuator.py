"""Actuator service client."""

from typing import Any, Dict, List, Optional

import grpc
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos import actuator_pb2, actuator_pb2_grpc
from kos.actuator_pb2 import CalibrateActuatorMetadata


class CalibrationStatus:
    Calibrating = "calibrating"
    Calibrated = "calibrated"
    Timeout = "timeout"


class CalibrationMetadata:
    def __init__(self, metadata_any: AnyPb2) -> None:
        self.actuator_id: Optional[int] = None
        self.status: Optional[str] = None
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

    def get_calibration_status(self, actuator_id: int) -> Optional[str]:
        response = self.operations_stub.GetOperation(
            operations_pb2.GetOperationRequest(name=f"operations/calibrate_actuator/{actuator_id}")
        )
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status

    def command_actuators(self, commands: List[Dict[str, Any]]) -> List[actuator_pb2.ActionResult]:
        """Command multiple actuators at once.

        Args:
            commands: List of dictionaries containing actuator commands.
                     Each dict should have 'actuator_id' and optionally 'position',
                     'velocity', and 'torque'.

        Returns:
            List of ActionResult objects indicating success/failure for each command.
        """
        actuator_commands = [actuator_pb2.ActuatorCommand(**cmd) for cmd in commands]
        request = actuator_pb2.CommandActuatorsRequest(commands=actuator_commands)
        response = self.stub.CommandActuators(request)
        return response.results

    def configure_actuator(self, actuator_id: int, **kwargs: Dict[str, Any]) -> actuator_pb2.ActionResponse:
        """Configure an actuator's parameters.

        Args:
            actuator_id: ID of the actuator to configure
            **kwargs: Configuration parameters that may include:
                     kp, kd, ki, max_torque, protective_torque,
                     protection_time, torque_enabled, new_actuator_id

        Returns:
            ActionResponse indicating success/failure
        """
        config = {"actuator_id": actuator_id, **kwargs}
        request = actuator_pb2.ConfigureActuatorRequest(**config)
        return self.stub.ConfigureActuator(request)

    def get_actuators_state(self, actuator_ids: List[int]) -> List[actuator_pb2.ActuatorStateResponse]:
        """Get the state of multiple actuators.

        Args:
            actuator_ids: List of actuator IDs to query

        Returns:
            List of ActuatorStateResponse objects containing the state information
        """
        request = actuator_pb2.GetActuatorsStateRequest(actuator_ids=actuator_ids)
        response = self.stub.GetActuatorsState(request)
        return response.states
