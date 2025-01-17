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


class ForwardResponse(TypedDict):
    """Response from running model inference.

    Args:
        outputs: Dictionary mapping tensor names to output tensors
        error: Optional error information if inference failed
    """
    outputs: dict[str, Tensor]
    error: NotRequired[common_pb2.Error]


class ModelInfo(TypedDict):
    """Information about a model.

    Args:
        uid: Model UID (assigned by server)
        metadata: Model metadata
        input_specs: Expected input tensor specifications
        output_specs: Expected output tensor specifications
        description: str
    """
    uid: str
    metadata: ModelMetadata
    input_specs: dict[str, Tensor]
    output_specs: dict[str, Tensor]
    description: str


class GetModelsInfoResponse(TypedDict):
    """Response containing information about available models."""
    models: list[ModelInfo]
    error: NotRequired[common_pb2.Error]


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

    def get_models_info(self, model_uids: list[str] | None = None) -> GetModelsInfoResponse:
        """Get information about available models.

        Args:
            model_uids: Optional list of specific model UIDs to get info for.
                       If None, returns info for all models.

        Returns:
            GetModelsInfoResponse containing:
                models: List of ModelInfo objects
                error: Optional error information if fetching failed
        """
        if model_uids is not None:
            request = inference_pb2.GetModelsInfoRequest(
                model_uids=inference_pb2.ModelUids(uids=model_uids)
            )
        else:
            request = inference_pb2.GetModelsInfoRequest(all=True)

        response = self.stub.GetModelsInfo(request)

        return GetModelsInfoResponse(
            models=[
                ModelInfo(
                    uid=model.uid,
                    metadata=ModelMetadata(
                        model_name=model.metadata.model_name if model.metadata.HasField("model_name") else None,
                        model_description=model.metadata.model_description if model.metadata.HasField("model_description") else None,
                        model_version=model.metadata.model_version if model.metadata.HasField("model_version") else None,
                        model_author=model.metadata.model_author if model.metadata.HasField("model_author") else None,
                    ),
                    input_specs={
                        name: Tensor(
                            values=list(tensor.values),
                            shape=[
                                TensorDimension(
                                    size=dim.size,
                                    name=dim.name,
                                    dynamic=dim.dynamic
                                )
                                for dim in tensor.shape
                            ]
                        )
                        for name, tensor in model.input_specs.items()
                    },
                    output_specs={
                        name: Tensor(
                            values=list(tensor.values),
                            shape=[
                                TensorDimension(
                                    size=dim.size,
                                    name=dim.name,
                                    dynamic=dim.dynamic
                                )
                                for dim in tensor.shape
                            ]
                        )
                        for name, tensor in model.output_specs.items()
                    },
                    description=model.description
                )
                for model in response.models
            ],
            error=response.error if response.HasField("error") else None
        )

    def forward(self, model_uid: str, inputs: dict[str, Tensor]) -> ForwardResponse:
        """Run inference using a specified model.

        Args:
            model_uid: The UID of the model to use for inference.
            inputs: Dictionary mapping tensor names to tensors.

        Returns:
            ForwardResponse containing:
                outputs: Dictionary mapping tensor names to output tensors
                error: Optional error information if inference failed
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
            proto_tensor = inference_pb2.Tensor(values=tensor["values"], shape=shape)
            tensor_inputs[name] = proto_tensor

        response = self.stub.Forward(
            inference_pb2.ForwardRequest(model_uid=model_uid, inputs=tensor_inputs)
        )

        return ForwardResponse(
            outputs={
                name: Tensor(
                    values=list(tensor.values),
                    shape=[
                        TensorDimension(
                            size=dim.size,
                            name=dim.name,
                            dynamic=dim.dynamic
                        )
                        for dim in tensor.shape
                    ]
                )
                for name, tensor in response.outputs.items()
            },
            error=response.error if response.HasField("error") else None
        )
