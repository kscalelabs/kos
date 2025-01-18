use crate::hal::Sound;
use crate::kos_proto::common::ActionResponse;
use crate::kos_proto::sound::sound_service_server::SoundService;
use crate::kos_proto::sound::*;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tracing::trace;

pub struct SoundServiceImpl {
    sound: Arc<dyn Sound>,
}

impl SoundServiceImpl {
    pub fn new(sound: Arc<dyn Sound>) -> Self {
        Self { sound }
    }
}

#[tonic::async_trait]
impl SoundService for SoundServiceImpl {
    async fn get_audio_info(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetAudioInfoResponse>, Status> {
        let info = self.sound.get_audio_info().await?;
        trace!("Getting audio info, response: {:?}", info);
        Ok(Response::new(info))
    }

    async fn play_audio(
        &self,
        request: Request<tonic::Streaming<PlayAudioRequest>>,
    ) -> Result<Response<ActionResponse>, Status> {
        let mut stream = request.into_inner();
        
        // Get the first message which must contain the config
        let first_msg = stream.message().await
            .map_err(|e| Status::internal(format!("Failed to receive first audio message: {:?}", e)))?
            .ok_or_else(|| Status::invalid_argument("Empty audio stream"))?;
            
        let config = first_msg.config
            .ok_or_else(|| Status::invalid_argument("First message must contain audio configuration"))?;

        trace!("Starting audio playback with config: {:?}", config);

        // Create channel for audio data
        let (tx, _rx) = mpsc::channel(32);
        
        // Start playback with the sender
        let response = self.sound.play_audio(config, tx.clone()).await?;
        
        // Spawn task to handle incoming audio data
        tokio::spawn(async move {
            while let Ok(Some(msg)) = stream.message().await {
                if let Err(e) = tx.send(Bytes::from(msg.audio_data)).await {
                    tracing::error!("Failed to send audio data: {:?}", e);
                    break;
                }
            }
        });

        Ok(Response::new(response))
    }

    type RecordAudioStream = Pin<Box<dyn Stream<Item = Result<RecordAudioResponse, Status>> + Send>>;

    async fn record_audio(
        &self,
        request: Request<RecordAudioRequest>,
    ) -> Result<Response<Self::RecordAudioStream>, Status> {
        let request = request.into_inner();
        
        trace!(
            "Starting audio recording with config: {:?}, duration: {}ms",
            request.config,
            request.duration_ms
        );

        let config = request.config
            .ok_or_else(|| Status::invalid_argument("Audio configuration is required"))?;

        let stream = self.sound.record_audio(config, request.duration_ms).await?;

        // Convert the stream into the expected response type
        let response_stream = async_stream::try_stream! {
            for await audio_data in stream {
                yield RecordAudioResponse {
                    audio_data: audio_data.to_vec(),
                    error: None,
                };
            }
        };

        Ok(Response::new(Box::pin(response_stream)))
    }

    async fn stop_recording(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ActionResponse>, Status> {
        let response = self.sound.stop_recording().await?;
        trace!("Stopping audio recording");
        Ok(Response::new(response))
    }
}
