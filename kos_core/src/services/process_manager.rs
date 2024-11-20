use crate::hal::ProcessManager;
use crate::kos_proto::process_manager::process_manager_service_server::ProcessManagerService;
use crate::kos_proto::process_manager::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct ProcessManagerServiceImpl {
    process_manager: Arc<dyn ProcessManager>,
}

impl ProcessManagerServiceImpl {
    pub fn new(process_manager: Arc<dyn ProcessManager>) -> Self {
        Self { process_manager }
    }
}

#[tonic::async_trait]
impl ProcessManagerService for ProcessManagerServiceImpl {
    async fn start_k_clip(
        &self,
        _request: Request<()>,
    ) -> Result<Response<KClipStartResponse>, Status> {
        trace!("Starting K-Clip");

        Ok(Response::new(
            self.process_manager
                .start_kclip()
                .await
                .map_err(|e| Status::internal(format!("Failed to start K-Clip, {:?}", e)))?,
        ))
    }

    async fn stop_k_clip(
        &self,
        _request: Request<()>,
    ) -> Result<Response<KClipStopResponse>, Status> {
        trace!("Stopping K-Clip");

        Ok(Response::new(
            self.process_manager
                .stop_kclip()
                .await
                .map_err(|e| Status::internal(format!("Failed to stop K-Clip, {:?}", e)))?,
        ))
    }
}
