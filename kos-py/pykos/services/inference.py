"""Inference service client."""

from typing import NotRequired, TypedDict

import grpc

from kos_protos import common_pb2, inference_pb2, inference_pb2_grpc
from kos_protos.inference_pb2 import (
    ForwardRequest,
    ForwardResponse,
    GetModelsInfoRequest,
    GetModelsInfoResponse,
    LoadModelsResponse,
    ModelMetadata as ProtoModelMetadata,
    ModelUids,
    Tensor as ProtoTensor,
    UploadModelResponse,
)


class ModelMetadata(TypedDict):
    """Model metadata for uploading models.

    Fields:
        model_name: Optional name of the model
        model_description: Optional description of the model
        model_version: Optional version of the model
        model_author: Optional author of the model
    """

    model_name: NotRequired[str | None]
    model_description: NotRequired[str | None]
    model_version: NotRequired[str | None]
    model_author: NotRequired[str | None]


class TensorDimension(TypedDict):
    """Information about a tensor dimension.

    Fields:
        size: Size of this dimension
        name: Name of this dimension (e.g., "batch", "channels", "height")
        dynamic: Whether this dimension can vary (e.g., batch size)
    """

    size: int
    name: str
    dynamic: bool


class Tensor(TypedDict):
    """A tensor containing data.

    Fields:
        values: Tensor values in row-major order
        shape: List of dimension information
    """

    values: list[float]
    shape: list[TensorDimension]


class InferenceServiceClient:
    """Client for the inference service.

    This client provides methods to interact with the inference service for
    uploading, loading, and running machine learning models.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the inference service client.

        Args:
            channel: gRPC channel for communication with the service
        """
        self.stub = inference_pb2_grpc.InferenceServiceStub(channel)

    def upload_model(self, model_data: bytes, metadata: ModelMetadata | None = None) -> UploadModelResponse:
        """Upload a model to the inference service.

        Args:
            model_data: The serialized model data
            metadata: Optional metadata about the model

        Returns:
            UploadModelResponse containing:
            - uid: Unique identifier assigned to the model
            - error: Optional error information
        """
        proto_metadata = None
        if metadata is not None:
            proto_metadata = ProtoModelMetadata(**metadata)
        request = inference_pb2.UploadModelRequest(model=model_data, metadata=proto_metadata)
        return self.stub.UploadModel(request)

    def load_models(self, uids: list[str]) -> LoadModelsResponse:
        """Load models into memory.

        Args:
            uids: List of model UIDs to load

        Returns:
            LoadModelsResponse containing:
            - models: List of loaded model information
            - result: Success/failure status
        """
        request = ModelUids(uids=uids)
        return self.stub.LoadModels(request)

    def unload_models(self, uids: list[str]) -> common_pb2.ActionResponse:
        """Unload models from memory.

        Args:
            uids: List of model UIDs to unload

        Returns:
            ActionResponse indicating if the unload was successful
        """
        request = ModelUids(uids=uids)
        return self.stub.UnloadModels(request)

    def get_models_info(self, model_uids: list[str] | None = None) -> GetModelsInfoResponse:
        """Get information about available models.

        Args:
            model_uids: Optional list of specific model UIDs to query.
                       If None, returns info for all available models.

        Returns:
            GetModelsInfoResponse containing:
            - models: List of ModelInfo objects with model details
            - error: Optional error information
        """
        if model_uids is not None:
            request = GetModelsInfoRequest(model_uids=ModelUids(uids=model_uids))
        else:
            request = GetModelsInfoRequest(all=True)
        return self.stub.GetModelsInfo(request)

    def forward(self, model_uid: str, inputs: dict[str, Tensor]) -> ForwardResponse:
        """Run inference on a model.

        Args:
            model_uid: The UID of the model to use
            inputs: Dictionary mapping input names to input tensors

        Returns:
            ForwardResponse containing:
            - outputs: Dictionary mapping tensor names to output tensors
            - error: Optional error information
        """
        tensor_inputs = {}
        for name, tensor in inputs.items():
            shape = [
                inference_pb2.Tensor.Dimension(size=dim["size"], name=dim["name"], dynamic=dim["dynamic"])
                for dim in tensor["shape"]
            ]
            proto_tensor = ProtoTensor(values=tensor["values"], shape=shape)
            tensor_inputs[name] = proto_tensor

        request = ForwardRequest(model_uid=model_uid, inputs=tensor_inputs)
        return self.stub.Forward(request)
