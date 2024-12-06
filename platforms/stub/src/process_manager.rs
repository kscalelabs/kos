use async_trait::async_trait;
use eyre::Result;
use kos_core::hal::{KClipStartResponse, KClipStopResponse, ProcessManager};
use kos_core::kos_proto::common::{Error, ErrorCode};
use std::sync::Mutex;
use uuid::Uuid;

pub struct StubProcessManager {
    kclip_uuid: Mutex<Option<String>>,
}

impl Default for StubProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StubProcessManager {
    pub fn new() -> Self {
        StubProcessManager {
            kclip_uuid: Mutex::new(None),
        }
    }
}

#[async_trait]
impl ProcessManager for StubProcessManager {
    async fn start_kclip(&self, _action: String) -> Result<KClipStartResponse> {
        let mut kclip_uuid = self.kclip_uuid.lock().unwrap();
        if kclip_uuid.is_some() {
            return Ok(KClipStartResponse {
                clip_uuid: None,
                error: Some(Error {
                    code: ErrorCode::InvalidArgument as i32,
                    message: "KClip is already started".to_string(),
                }),
            });
        }

        let new_uuid = Uuid::new_v4().to_string();
        *kclip_uuid = Some(new_uuid.clone());

        Ok(KClipStartResponse {
            clip_uuid: Some(new_uuid),
            error: None,
        })
    }

    async fn stop_kclip(&self) -> Result<KClipStopResponse> {
        let mut kclip_uuid = self.kclip_uuid.lock().unwrap();
        if kclip_uuid.is_none() {
            return Ok(KClipStopResponse {
                clip_uuid: None,
                error: Some(Error {
                    code: ErrorCode::InvalidArgument as i32,
                    message: "KClip is not running".to_string(),
                }),
            });
        }

        let stopped_uuid = kclip_uuid.take().unwrap();

        Ok(KClipStopResponse {
            clip_uuid: Some(stopped_uuid),
            error: None,
        })
    }
}
