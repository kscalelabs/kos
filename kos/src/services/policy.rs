use crate::hal::Policy;
use crate::kos_proto::policy::policy_service_server::PolicyService;
use crate::kos_proto::policy::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct PolicyServiceImpl {
    policy: Arc<dyn Policy>,
}

impl PolicyServiceImpl {
    pub fn new(policy: Arc<dyn Policy>) -> Self {
        Self { policy }
    }
}

#[tonic::async_trait]
impl PolicyService for PolicyServiceImpl {
    async fn start_policy(
        &self,
        request: Request<StartPolicyRequest>,
    ) -> Result<Response<StartPolicyResponse>, Status> {
        trace!("Starting Policy");
        let req = request.get_ref();

        Ok(Response::new(
            self.policy
                .start_policy(
                    req.action.clone(),
                    req.action_scale,
                    req.episode_length,
                    req.dry_run,
                )
                .await
                .map_err(|e| Status::internal(format!("Failed to start policy: {:?}", e)))?,
        ))
    }

    async fn stop_policy(
        &self,
        _request: Request<()>,
    ) -> Result<Response<StopPolicyResponse>, Status> {
        trace!("Stopping Policy");

        Ok(Response::new(self.policy.stop_policy().await.map_err(
            |e| Status::internal(format!("Failed to stop policy: {:?}", e)),
        )?))
    }

    async fn get_state(&self, _request: Request<()>) -> Result<Response<GetStateResponse>, Status> {
        trace!("Getting Policy State");

        Ok(Response::new(self.policy.get_state().await.map_err(
            |e| Status::internal(format!("Failed to get policy state: {:?}", e)),
        )?))
    }
}
