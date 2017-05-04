extern crate winapi;
extern crate kernel32;
extern crate futures;
extern crate tokio_core;

pub mod audio_client;
pub mod devices;
pub mod session;
mod util;
mod graph;

pub fn run(capture_client: audio_client::AudioClient) {
    use futures::stream::Stream;
    use futures::sink::Sink;
    use tokio_core::reactor::Core;
    use ::graph::capture_node::CaptureNode;
    use ::graph::render_node::RenderNode;

    let mut core = Core::new().unwrap();

    let capture_node = CaptureNode::new(capture_client).unwrap();
    let render_node = RenderNode::new();

    let process = render_node.send_all(capture_node);

    core.run(process).unwrap();
}
