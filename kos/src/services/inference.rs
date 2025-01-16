use crate::hal::{Inference, Tensor as HALTensor, Dimension};
use crate::kos_proto::common::ActionResponse;
use crate::kos_proto::inference::inference_service_server::InferenceService;
use crate::kos_proto::inference::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct InferenceServiceImpl {
    inference: Arc<dyn Inference>,
}

impl InferenceServiceImpl {
    pub fn new(inference: Arc<dyn Inference>) -> Self {
        Self { inference }
    }
}

#[tonic::async_trait]
impl InferenceService for InferenceServiceImpl {
    async fn upload_model(
        &self,
        request: Request<UploadModelRequest>,
    ) -> Result<Response<UploadModelResponse>, Status> {
        trace!("upload_model request received");
        let request = request.into_inner();
        let model_data = request.model;
        let metadata: Option<ModelMetadata> = request.metadata;
        self.inference
            .upload_model(model_data, metadata)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Failed to upload model: {:?}", e)))
    }

    async fn load_models(
        &self,
        request: Request<ModelUids>,
    ) -> Result<Response<LoadModelsResponse>, Status> {
        trace!("load_models request received");
        let request = request.into_inner();
        self.inference
            .load_models(request.uids)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Failed to load models: {:?}", e)))
    }

    async fn unload_models(
        &self,
        request: Request<ModelUids>,
    ) -> Result<Response<ActionResponse>, Status> {
        trace!("unload_models request received");
        let request = request.into_inner();
        self.inference
            .unload_models(request.uids)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Failed to unload models: {:?}", e)))
    }

    async fn get_models_info(
        &self,
        request: Request<GetModelsInfoRequest>,
    ) -> Result<Response<GetModelsInfoResponse>, Status> {
        trace!("get_models_info request received");
        let request = request.into_inner();
        self.inference
            .get_models_info(request)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Failed to get models info: {:?}", e)))
    }

    async fn forward(
        &self,
        request: Request<ForwardRequest>,
    ) -> Result<Response<ForwardResponse>, Status> {
        trace!("forward request received");
        let request = request.into_inner();

        // Convert proto tensors to HAL tensors
        let inputs: std::collections::HashMap<String, HALTensor> = request
            .inputs
            .into_iter()
            .map(|(name, proto_tensor)| {
                (
                    name,
                    HALTensor {
                        values: proto_tensor.values,
                        shape: proto_tensor.shape.into_iter().map(|d| Dimension {
                            size: d.size,
                            name: d.name,
                            dynamic: d.dynamic,
                        }).collect(),
                    },
                )
            })
            .collect();

        self.inference
            .forward(request.model_uid, inputs)
            .await
            .map(Response::new)
            .map_err(|e| Status::internal(format!("Failed to run inference: {:?}", e)))
    }
}
