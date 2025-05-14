use async_trait::async_trait;
use eyre::Result;
use kos::hal::{GetStateResponse, Policy, StartPolicyResponse, StopPolicyResponse};
use kos::kos_proto::common::{Error, ErrorCode};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct StubPolicy {
    policy_uuid: Mutex<Option<String>>,
    state: Mutex<HashMap<String, String>>,
}

impl Default for StubPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl StubPolicy {
    pub fn new() -> Self {
        StubPolicy {
            policy_uuid: Mutex::new(None),
            state: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Policy for StubPolicy {
    async fn start_policy(
        &self,
        action: String,
        action_scale: f32,
        episode_length: i32,
        dry_run: bool,
    ) -> Result<StartPolicyResponse> {
        let mut policy_uuid = self.policy_uuid.lock().unwrap();
        if policy_uuid.is_some() {
            return Ok(StartPolicyResponse {
                policy_uuid: None,
                error: Some(Error {
                    code: ErrorCode::InvalidArgument as i32,
                    message: "Policy is already running".to_string(),
                }),
            });
        }

        let new_uuid = Uuid::new_v4().to_string();
        *policy_uuid = Some(new_uuid.clone());

        // Update state with policy parameters
        let mut state = self.state.lock().unwrap();
        state.insert("action".to_string(), action);
        state.insert("action_scale".to_string(), action_scale.to_string());
        state.insert("episode_length".to_string(), episode_length.to_string());
        state.insert("dry_run".to_string(), dry_run.to_string());

        Ok(StartPolicyResponse {
            policy_uuid: Some(new_uuid),
            error: None,
        })
    }

    async fn stop_policy(&self) -> Result<StopPolicyResponse> {
        let mut policy_uuid = self.policy_uuid.lock().unwrap();
        if policy_uuid.is_none() {
            return Ok(StopPolicyResponse {
                policy_uuid: None,
                error: Some(Error {
                    code: ErrorCode::InvalidArgument as i32,
                    message: "Policy is not running".to_string(),
                }),
            });
        }

        let stopped_uuid = policy_uuid.take().unwrap();

        // Clear the state when stopping
        let mut state = self.state.lock().unwrap();
        state.clear();

        Ok(StopPolicyResponse {
            policy_uuid: Some(stopped_uuid),
            error: None,
        })
    }

    async fn get_state(&self) -> Result<GetStateResponse> {
        let state = self.state.lock().unwrap();
        Ok(GetStateResponse {
            state: state.clone(),
            error: None,
        })
    }
}
