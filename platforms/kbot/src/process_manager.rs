use async_trait::async_trait;
use eyre::{eyre, Result, WrapErr};
use kos_core::hal::{KClipStartResponse, KClipStopResponse, ProcessManager};
use kos_core::kos_proto::common::{Error, ErrorCode};
use std::sync::Mutex;
use uuid::Uuid;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;

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

    fn create_pipeline(uuid: &str) -> Result<(gst::Pipeline, gst::Element)> {
        gst::init().wrap_err("Failed to initialize GStreamer")?;
        
        let pipeline = gst::Pipeline::new(None);
        
        // Create elements
        let src = gst::ElementFactory::make("videotestsrc")
            .name("src")
            .property_from_str("pattern", "smpte")
            .property("is-live", true)
            .build()
            .wrap_err("Failed to create videotestsrc")?;

        let videorate = gst::ElementFactory::make("videorate")
            .name("videorate0")
            .build()
            .wrap_err("Failed to create videorate")?;

        let capsfilter = gst::ElementFactory::make("capsfilter")
            .name("capsfilter0")
            .property(
                "caps",
                gst::Caps::builder("video/x-raw")
                    .field("width", 640i32)
                    .field("height", 480i32)
                    .field("framerate", gst::Fraction::new(30, 1))
                    .build(),
            )
            .build()
            .wrap_err("Failed to create capsfilter")?;

        let nvvidconv = gst::ElementFactory::make("nvvidconv")
            .name("nvvidconv0")
            .build()
            .wrap_err("Failed to create nvvidconv")?;

        let nvvidconv_caps = gst::ElementFactory::make("capsfilter")
            .name("nvvidconv_caps")
            .property(
                "caps",
                gst::Caps::builder("video/x-raw")
                    .features(["memory:NVMM"])
                    .field("format", "NV12")
                    .build(),
            )
            .build()
            .wrap_err("Failed to create nvvidconv capsfilter")?;

        let tee = gst::ElementFactory::make("tee")
            .name("tee0")
            .build()
            .wrap_err("Failed to create tee")?;

        let queue_monitor = gst::ElementFactory::make("queue")
            .name("queue_monitor")
            .build()
            .wrap_err("Failed to create monitor queue")?;

        let queue_record = gst::ElementFactory::make("queue")
            .name("queue_record")
            .build()
            .wrap_err("Failed to create record queue")?;

        let appsink = gst_app::AppSink::builder()
            .name("appsink0")
            .caps(&gst::Caps::builder("video/x-raw")
                .features(["memory:NVMM"])
                .field("format", "NV12")
                .build())
            .build();

        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Error)?;
                    let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                    
                    let pts = buffer.pts();
                    tracing::debug!("New frame PTS: {:?}", pts);

                    Ok(gst::FlowSuccess::Ok)
                })
                .build()
        );

        let encoder = gst::ElementFactory::make("nvv4l2h265enc")
            .name("nvv4l2h265enc0")
            .build()
            .wrap_err("Failed to create H265 encoder")?;

        let parser = gst::ElementFactory::make("h265parse")
            .name("h265parse0")
            .build()
            .wrap_err("Failed to create H265 parser")?;

        let muxer = gst::ElementFactory::make("qtmux")
            .name("qtmux0")
            .build()?;

        let sink = gst::ElementFactory::make("filesink")
            .name("filesink0")
            .property("location", format!("out_{}.mov", uuid))
            .build()?;

        // Add elements to pipeline
        pipeline.add_many(&[
            &src,
            &videorate,
            &capsfilter,
            &nvvidconv,
            &nvvidconv_caps,
            &tee,
            &queue_monitor,
            &queue_record,
            &appsink.upcast_ref(),
            &encoder,
            &parser,
            &muxer,
            &sink,
        ])?;
        
        // Link elements up to tee
        gst::Element::link_many(&[
            &src,
            &videorate,
            &capsfilter,
            &nvvidconv,
            &nvvidconv_caps,
            &tee,
        ])?;

        // Link monitoring branch
        gst::Element::link_many(&[
            &queue_monitor,
            &appsink.upcast_ref(),
        ])?;

        // Link recording branch
        gst::Element::link_many(&[
            &queue_record,
            &encoder,
            &parser,
            &muxer,
            &sink,
        ])?;

        // Link tee to both queues using proper pad names
        tee.link_pads(Some("src_%u"), &queue_monitor, None)?;
        tee.link_pads(Some("src_%u"), &queue_record, None)?;

        Ok((pipeline, sink))
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

        let (pipeline, _sink) = Self::create_pipeline(&new_uuid)?;
        
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
        let mut pipeline_guard = self.pipeline.lock().unwrap();
        
        if let Some(pipeline) = pipeline_guard.as_ref() {
            // Get the bus
            let bus = pipeline.bus().ok_or_else(|| eyre!("Failed to get pipeline bus"))?;
            
            // Send EOS event
            pipeline.send_event(gst::event::Eos::new());
            
            // Wait for EOS or Error message with timeout
            let timeout = gst::ClockTime::from_seconds(5);
            let msg = bus.timed_pop_filtered(timeout, &[gst::MessageType::Eos, gst::MessageType::Error]);
            
            match msg {
                Some(msg) => {
                    use gst::MessageView;
                    match msg.view() {
                        MessageView::Eos(..) => {
                            tracing::info!("Pipeline received EOS");
                        }
                        MessageView::Error(err) => {
                            return Ok(KClipStopResponse {
                                clip_uuid: None,
                                error: Some(Error {
                                    code: ErrorCode::Unknown as i32,
                                    message: format!("Pipeline error: {} ({})", 
                                        err.error(), 
                                        err.debug().unwrap_or_default()),
                                }),
                            });
                        }
                        _ => unreachable!(),
                    }
                }
                None => {
                    tracing::warn!("Timeout waiting for pipeline EOS");
                }
            }

            // Change state to NULL
            let state_change = pipeline
                .set_state(gst::State::Null)
                .map_err(|e| eyre!("Failed to set pipeline state to Null: {}", e))?;

            if !matches!(state_change, gst::StateChangeSuccess::Success) {
                return Ok(KClipStopResponse {
                    clip_uuid: None,
                    error: Some(Error {
                        code: ErrorCode::Unknown as i32,
                        message: "Failed to change pipeline state to Null".to_string(),
                    }),
                });
            }

            // Get the UUID before clearing
            let uuid = self.kclip_uuid.lock().unwrap().take();
            
            // Clear the pipeline
            *pipeline_guard = None;

            Ok(KClipStopResponse {
                clip_uuid: uuid,
                error: None,
            })
        } else {
            Ok(KClipStopResponse {
                clip_uuid: None,
                error: Some(Error {
                    code: ErrorCode::InvalidArgument as i32,
                    message: "No active KClip recording".to_string(),
                }),
            })
        }
    }
}
