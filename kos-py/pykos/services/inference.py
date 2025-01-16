"""Inference service client."""

from typing import NotRequired, TypedDict

import grpc

from kos_protos import common_pb2, inference_pb2, inference_pb2_grpc


class ModelMetadata(TypedDict):
    """Model metadata for uploading models.

    All fields are optional and can be used to provide additional information about the model.
    """

    model_name: NotRequired[str]
    model_description: NotRequired[str]
    model_version: NotRequired[str]
    model_author: NotRequired[str]


class TensorDimension(TypedDict):
    """Information about a tensor dimension.

    Args:
        size: Size of this dimension
        name: Name of this dimension (e.g., "batch", "channels", "height")
        dynamic: Whether this dimension can vary (e.g., batch size)
    """
    size: int
    name: str
    dynamic: bool


class Tensor(TypedDict):
    """A tensor containing data.

    Args:
        values: Tensor values in row-major order
        shape: List of dimension information
    """
    values: list[float]
    shape: list[TensorDimension]


class InferenceServiceClient:
    """Client for the InferenceService.

    This service allows uploading models and running inference on them.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        """Initialize the inference service client.

        Args:
            channel: gRPC channel to use for communication.
        """
        self.stub = inference_pb2_grpc.InferenceServiceStub(channel)

    def upload_model(
        self, model_data: bytes, metadata: ModelMetadata | None = None
    ) -> inference_pb2.UploadModelResponse:
        """Upload a model to the robot.

        Example:
        >>> client.upload_model(model_data,
        ... metadata={"model_name": "MyModel",
        ... "model_description": "A model for inference",
        ... "model_version": "1.0.0",
        ... "model_author": "John Doe"})

        Args:
            model_data: The binary model data to upload.
            metadata: Optional metadata about the model that can include:
                     model_name: Name of the model
                     model_description: Description of the model
                     model_version: Version of the model
                     model_author: Author of the model

        Returns:
            UploadModelResponse containing the model UID and any error information.
        """
        proto_metadata = None
        if metadata is not None:
            proto_metadata = inference_pb2.ModelMetadata(**metadata)
        request = inference_pb2.UploadModelRequest(model=model_data, metadata=proto_metadata)
        return self.stub.UploadModel(request)

    def load_models(self, uids: list[str]) -> inference_pb2.LoadModelsResponse:
        """Load models from the robot's filesystem.

        Args:
            uids: List of model UIDs to load.

        Returns:
            LoadModelsResponse containing information about the loaded models.
        """
        request = inference_pb2.ModelUids(uids=uids)
        return self.stub.LoadModels(request)

    def unload_models(self, uids: list[str]) -> common_pb2.ActionResponse:
        """Unload models from the robot's filesystem.

        Args:
            uids: List of model UIDs to unload.

        Returns:
            ActionResponse indicating success/failure of the unload operation.
        """
        request = inference_pb2.ModelUids(uids=uids)
        return self.stub.UnloadModels(request)

    def get_models_info(self, model_uids: list[str] | None = None) -> inference_pb2.GetModelsInfoResponse:
        """Get information about available models.

        Args:
            model_uids: Optional list of specific model UIDs to get info for.
                       If None, returns info for all models.

        Returns:
            GetModelsInfoResponse containing information about the requested models.
        """
        if model_uids is not None:
            request = inference_pb2.GetModelsInfoRequest(model_uids=inference_pb2.ModelUids(uids=model_uids))
        else:
            request = inference_pb2.GetModelsInfoRequest(all=True)
        return self.stub.GetModelsInfo(request)

    def forward(
        self,
        model_uid: str,
        inputs: dict[str, Tensor]
    ) -> inference_pb2.ForwardResponse:
        """Run inference using a specified model.

        Args:
            model_uid: The UID of the model to use for inference.
            inputs: Dictionary mapping tensor names to tensors.

        Returns:
            ForwardResponse containing the named model outputs and any error information.
        """
        tensor_inputs = {}
        for name, tensor in inputs.items():
            shape = [
                inference_pb2.Tensor.Dimension(
                    size=dim["size"],
                    name=dim["name"],
                    dynamic=dim["dynamic"]
                )
                for dim in tensor["shape"]
            ]
            proto_tensor = inference_pb2.Tensor(
                values=tensor["values"],
                shape=shape
            )
            tensor_inputs[name] = proto_tensor

        request = inference_pb2.ForwardRequest(model_uid=model_uid, inputs=tensor_inputs)
        return self.stub.Forward(request)
