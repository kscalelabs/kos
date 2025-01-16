"""Inference service client."""

from typing import List, Optional

import grpc

from kos_protos import common_pb2, inference_pb2, inference_pb2_grpc


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
        self, model_data: bytes, metadata: Optional[inference_pb2.ModelMetadata] = None
    ) -> inference_pb2.UploadModelResponse:
        """Upload a model to the robot.

        Args:
            model_data: The binary model data to upload.
            metadata: Optional metadata about the model (name, description, version, author).

        Returns:
            UploadModelResponse containing the model UID and any error information.
        """
        request = inference_pb2.UploadModelRequest(model=model_data, metadata=metadata)
        return self.stub.UploadModel(request)

    def load_models(self, uids: List[str]) -> inference_pb2.LoadModelsResponse:
        """Load models from the robot's filesystem.

        Args:
            uids: List of model UIDs to load.

        Returns:
            LoadModelsResponse containing information about the loaded models.
        """
        request = inference_pb2.ModelUids(uids=uids)
        return self.stub.LoadModels(request)

    def unload_models(self, uids: List[str]) -> common_pb2.ActionResponse:
        """Unload models from the robot's filesystem.

        Args:
            uids: List of model UIDs to unload.

        Returns:
            ActionResponse indicating success/failure of the unload operation.
        """
        request = inference_pb2.ModelUids(uids=uids)
        return self.stub.UnloadModels(request)

    def get_models_info(self, model_uids: Optional[List[str]] = None) -> inference_pb2.GetModelsInfoResponse:
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

    def forward(self, model_uid: str, inputs: List[float]) -> inference_pb2.ForwardResponse:
        """Run inference using a specified model.

        Args:
            model_uid: The UID of the model to use for inference.
            inputs: List of input values for the model.

        Returns:
            ForwardResponse containing the model outputs and any error information.
        """
        request = inference_pb2.ForwardRequest(model_uid=model_uid, inputs=inputs)
        return self.stub.Forward(request)
