use async_trait::async_trait;
use eyre::Result;
use kos::hal::Speech;
use kos::kos_proto::speech::SynthesizeResponse;
use std::process::Command;
use uuid::Uuid;
pub struct StubSpeech {}

impl Default for StubSpeech {
    fn default() -> Self {
        Self::new()
    }
}

impl StubSpeech {
    pub fn new() -> Self {
        StubSpeech {}
    }
}

#[async_trait]
impl Speech for StubSpeech {
    async fn synthesize(&self, text: String) -> Result<SynthesizeResponse> {
        // Generate a unique filename for the wav output
        let output_file = format!("synthesize_{}.wav", Uuid::new_v4());

        Ok(SynthesizeResponse {
            file_path: output_file,
            error: None,
        })
    }
}
