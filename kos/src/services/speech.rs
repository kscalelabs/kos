use crate::hal::Speech;
use crate::kos_proto::speech::speech_service_server::SpeechService;
use crate::kos_proto::speech::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct SpeechServiceImpl {
    speech: Arc<dyn Speech>,
}

impl SpeechServiceImpl {
    pub fn new(speech: Arc<dyn Speech>) -> Self {
        Self { speech }
    }
}

#[tonic::async_trait]
impl SpeechService for SpeechServiceImpl {
    async fn synthesize(
        &self,
        request: Request<SynthesizeRequest>,
    ) -> Result<Response<SynthesizeResponse>, Status> {
        let req = request.into_inner();
        trace!("Synthesizing text: {}", req.text);


        Ok(Response::new(
            self
            .speech
            .synthesize(req.text)
            .await
            .map_err(|e| Status::internal(format!("Failed to synthesize text, {:?}", e)))?,
        ))
    }
    
    async fn transcribe(
        &self,
        request: Request<TranscribeRequest>,
    ) -> Result<Response<TranscribeResponse>, Status> {
        Err(Status::unimplemented("Transcribe not implemented"))
    }
}