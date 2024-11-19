use gstreamer::prelude::*;
use gstreamer::{Pipeline};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Create pipeline
    let pipeline = Pipeline::new();

    // Create elements
    let videotestsrc = gstreamer::ElementFactory::make("videotestsrc").build()?;
    let timeoverlay = gstreamer::ElementFactory::make("timeoverlay").build()?;
    let videoconvert = gstreamer::ElementFactory::make("videoconvert").build()?;
    let x265enc = gstreamer::ElementFactory::make("x265enc").build()?;
    let mpegtsmux = gstreamer::ElementFactory::make("mpegtsmux").build()?;
    let filesink = gstreamer::ElementFactory::make("filesink").build()?;
    filesink.set_property("location", "output.ts");

    // Add elements to pipeline
    pipeline.add_many(&[&videotestsrc, &timeoverlay, &videoconvert, &x265enc, &mpegtsmux, &filesink])?;

    // Link elements
    gstreamer::Element::link_many(&[&videotestsrc, &timeoverlay, &videoconvert, &x265enc, &mpegtsmux, &filesink])?;

    // Set pipeline state to playing
    pipeline.set_state(gstreamer::State::Playing)?;

    // Create a main loop to run the pipeline
    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();

    // Handle pipeline messages
    let bus = pipeline.bus().unwrap();
    let _bus_watch = bus.add_watch(move |_, msg| {
        use gstreamer::MessageView;
        match msg.view() {
            MessageView::Eos(..) => main_loop_clone.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                main_loop_clone.quit();
            }
            _ => (),
        }
        gstreamer::glib::ControlFlow::Continue
    })?;

    // Add periodic metadata update (every 20ms)
    let pipeline_weak = pipeline.downgrade();
    glib::timeout_add_local(Duration::from_millis(20), move || {
        if let Some(_pipeline) = pipeline_weak.upgrade() {
            // Add your custom metadata processing here
        }
        glib::Continue(true)
    });

    // Run the main loop
    main_loop.run();

    // Clean up
    pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
