"""Actuator service client."""

from typing import Any, Dict, List, NotRequired, Optional, TypedDict

import grpc
import grpc.aio
from google.longrunning import operations_pb2, operations_pb2_grpc
from google.protobuf.any_pb2 import Any as AnyPb2

from kos_protos import actuator_pb2, actuator_pb2_grpc, common_pb2
from kos_protos.actuator_pb2 import CalibrateActuatorMetadata


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
    def __init__(self, channel: grpc.aio.Channel) -> None:
        self.stub = actuator_pb2_grpc.ActuatorServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    async def calibrate(self, actuator_id: int) -> CalibrationMetadata:
        """Calibrate an actuator."""
        response = await self.stub.CalibrateActuator(actuator_pb2.CalibrateActuatorRequest(actuator_id=actuator_id))
        metadata = CalibrationMetadata(response.metadata)
        return metadata

    async def get_calibration_status(self, actuator_id: int) -> Optional[str]:
        response = await self.operations_stub.GetOperation(
            operations_pb2.GetOperationRequest(name=f"operations/calibrate_actuator/{actuator_id}")
        )
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status

    async def command_actuators(self, commands: List[Dict[str, Any]]) -> List[common_pb2.ActionResult]:
        """Command multiple actuators at once."""
        actuator_commands = [actuator_pb2.ActuatorCommand(**cmd) for cmd in commands]
        request = actuator_pb2.CommandActuatorsRequest(commands=actuator_commands)
        response = await self.stub.CommandActuators(request)
        return response.results

    async def configure_actuator(self, actuator_id: int, **kwargs: Dict[str, Any]) -> common_pb2.ActionResult:
        """Configure an actuator's parameters."""
        config = {"actuator_id": actuator_id, **kwargs}
        request = actuator_pb2.ConfigureActuatorRequest(**config)
        return await self.stub.ConfigureActuator(request)

    async def get_actuators_state(self, actuator_ids: Optional[List[int]] = None) -> List[common_pb2.ActionResult]:
        """Get the state of multiple actuators."""
        request = actuator_pb2.GetActuatorsStateRequest(actuator_ids=actuator_ids or [])
        response = await self.stub.GetActuatorsState(request)
        if actuator_ids is None:
            return response.states

        states = []
        for state in response.states:
            if state.actuator_id in actuator_ids:
                states.append(state)
        return states
