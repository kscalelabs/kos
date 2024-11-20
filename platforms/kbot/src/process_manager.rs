use async_trait::async_trait;
use eyre::Result;
use kos_core::hal::{KClipStartResponse, KClipStopResponse, ProcessManager};
use kos_core::kos_proto::common::{Error, ErrorCode};
use std::sync::Mutex;
use uuid::Uuid;
use gstreamer as gst;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use gstreamer::prelude::*;

pub struct KBotProcessManager {
    kclip_uuid: Mutex<Option<String>>,
    pipeline: Mutex<Option<gst::Pipeline>>,
}

impl KBotProcessManager {
    pub fn new() -> Self {
        gst::init().unwrap();
        
        KBotProcessManager {
            kclip_uuid: Mutex::new(None),
            pipeline: Mutex::new(None),
        }
    }

    fn create_pipeline() -> Result<(gst::Pipeline, gst_app::AppSink)> {
        let pipeline = gst::Pipeline::new(None);
        
        // Create elements
        let src = gst::ElementFactory::make("videotestsrc")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create videotestsrc: {}", e))?;
        src.set_property("pattern", "smpte");

        let capsfilter = gst::ElementFactory::make("capsfilter")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create capsfilter: {}", e))?;
        capsfilter.set_property(
            "caps",
            gst::Caps::builder("video/x-raw")
                .field("width", 1920i32)
                .field("height", 1080i32)
                .build(),
        );

        let encoder = gst::ElementFactory::make("nvv4l2h265enc")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create encoder: {}", e))?;
        
        let parser = gst::ElementFactory::make("h265parse")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create parser: {}", e))?;
        
        let muxer = gst::ElementFactory::make("qtmux")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create muxer: {}", e))?;
        
        let sink = gst::ElementFactory::make("filesink")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create filesink: {}", e))?;
        sink.set_property("location", "output_test_video.mov");
        
        // Create appsink for frame callbacks
        let appsink = gst::ElementFactory::make("appsink")
            .name("frame_sink")
            .build()
            .map_err(|e| eyre::eyre!("Failed to create appsink: {}", e))?;
        let appsink = appsink.dynamic_cast::<gst_app::AppSink>()
            .map_err(|_| eyre::eyre!("Failed to cast element to AppSink"))?;
        
        // Set up callbacks
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    if let Ok(sample) = appsink.pull_sample() {
                        if let Some(buffer) = sample.buffer() {
                            // Get timestamp
                            let pts = buffer.pts().unwrap_or(gst::ClockTime::ZERO);
                            tracing::trace!("New frame received with timestamp: {:?}", pts);
                        }
                    }
                    Ok(gst::FlowSuccess::Ok)
                })
                .build()
        );

        // Add elements to pipeline
        pipeline.add_many(&[
            &src,
            &capsfilter,
            &encoder,
            &parser,
            &muxer,
            &sink,
            appsink.upcast_ref(),
        ])?;
        
        // Link elements
        gst::Element::link_many(&[
            &src,
            &capsfilter,
            &encoder,
            &parser,
            &muxer,
            &sink,
        ])?;

        Ok((pipeline, appsink))
    }
}

#[async_trait]
impl ProcessManager for KBotProcessManager {
    async fn start_kclip(&self) -> Result<KClipStartResponse> {
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

        let (pipeline, appsink) = Self::create_pipeline()?;
        
        // Start the pipeline
        pipeline.set_state(gst::State::Playing)?;
        
        let mut pipeline_guard = self.pipeline.lock().unwrap();
        *pipeline_guard = Some(pipeline);

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

        let mut pipeline_guard = self.pipeline.lock().unwrap();
        if let Some(pipeline) = pipeline_guard.take() {
            pipeline.set_state(gst::State::Null)?;
        }

        let stopped_uuid = kclip_uuid.take().unwrap();

        Ok(KClipStopResponse {
            clip_uuid: Some(stopped_uuid),
            error: None,
        })
    }
}
