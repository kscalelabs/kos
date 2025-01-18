use crate::hal::LEDMatrix;
use crate::kos_proto::common::ActionResponse;
use crate::kos_proto::led_matrix::led_matrix_service_server::LedMatrixService;
use crate::kos_proto::led_matrix::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct LEDMatrixServiceImpl {
    led_matrix: Arc<dyn LEDMatrix>,
}

impl LEDMatrixServiceImpl {
    pub fn new(led_matrix: Arc<dyn LEDMatrix>) -> Self {
        Self { led_matrix }
    }
}

#[tonic::async_trait]
impl LedMatrixService for LEDMatrixServiceImpl {
    async fn get_matrix_info(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetMatrixInfoResponse>, Status> {
        let info = self
            .led_matrix
            .get_matrix_info()
            .await
            .map_err(|e| Status::internal(format!("Failed to get matrix info, {:?}", e)))?;

        trace!("Getting matrix info, response: {:?}", info);
        Ok(Response::new(info))
    }

    async fn write_buffer(
        &self,
        request: Request<WriteBufferRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        let buffer = request.into_inner().buffer;
        
        let response = self
            .led_matrix
            .write_buffer(buffer.clone())
            .await
            .map_err(|e| Status::internal(format!("Failed to write buffer, {:?}", e)))?;

        trace!("Writing LED buffer, buffer length: {}", buffer.len());
        Ok(Response::new(response))
    }

    async fn write_color_buffer(
        &self,
        request: Request<WriteColorBufferRequest>,
    ) -> Result<Response<ActionResponse>, Status> {
        let request = request.into_inner();
        
        let response = self
            .led_matrix
            .write_color_buffer(
                request.buffer,
                request.width,
                request.height,
                request.format.clone(),
                request.brightness,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to write color buffer, {:?}", e)))?;

        trace!(
            "Writing color buffer, dimensions: {}x{}, format: {}, brightness: {}",
            request.width,
            request.height,
            request.format,
            request.brightness
        );
        Ok(Response::new(response))
    }
}
