use async_trait::async_trait;
use kos::hal::Speech;
use std::process::Command;
use uuid::Uuid;
use kos::kos_proto::speech::SynthesizeResponse;


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
        let output_file = format!("/tmp/speech_{}.wav", Uuid::new_v4());
        
        // Create espeak command
        Command::new("espeak")
            .args([
                "-w",
                &output_file,
                "-v",
                "en",
                &text,
            ]);

        Ok(SynthesizeResponse {
            output_file: output_file,
            error: None,
        })
    }
}
