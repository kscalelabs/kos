from kos import actuator_pb2_grpc, actuator_pb2
from google.protobuf.empty_pb2 import Empty
from google.protobuf.any_pb2 import Any
from google.longrunning import operations_pb2, operations_pb2_grpc
from kos.actuator_pb2 import CalibrateActuatorMetadata

class CalibrationStatus:
    Calibrating = "calibrating"
    Calibrated = "calibrated"
    Timeout = "timeout"

class CalibrationMetadata:
    def __init__(self, metadata_any: Any):
        self.actuator_id = None
        self.status = None
        self.decode_metadata(metadata_any)

    def decode_metadata(self, metadata_any: Any):
        metadata = CalibrateActuatorMetadata()
        if metadata_any.Is(CalibrateActuatorMetadata.DESCRIPTOR):
            metadata_any.Unpack(metadata)
            self.actuator_id = metadata.actuator_id
            self.status = metadata.status

    def __str__(self):
        return f"CalibrationMetadata(actuator_id={self.actuator_id}, status={self.status})"
    
    def __repr__(self):
        return self.__str__()

class ActuatorServiceClient:
    def __init__(self, channel):
        self.stub = actuator_pb2_grpc.ActuatorServiceStub(channel)
        self.operations_stub = operations_pb2_grpc.OperationsStub(channel)

    def calibrate(self, actuator_id: int):
        """
        Calibrate an actuator.

        Returns:
            Operation: The operation for the calibration.
        """
        response = self.stub.CalibrateActuator(actuator_pb2.CalibrateActuatorRequest(actuator_id=actuator_id))
        metadata = CalibrationMetadata(response.metadata)
        return metadata

    def get_calibration_status(self, actuator_id: int):
        response = self.operations_stub.GetOperation(operations_pb2.GetOperationRequest(name=f"operations/calibrate_actuator/{actuator_id}"))
        metadata = CalibrationMetadata(response.metadata)
        return metadata.status
