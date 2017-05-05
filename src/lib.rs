extern crate byteorder;
extern crate bytes;
extern crate winapi;
extern crate kernel32;
extern crate futures;
extern crate tokio_core;
extern crate sample;

pub mod audio_client;
pub mod devices;
pub mod session;
mod util;
pub mod graph;

pub fn run(capture_client: audio_client::AudioClient, render_client: audio_client::AudioClient) {
    use futures::stream::Stream;
    use futures::sink::Sink;
    use tokio_core::reactor::Core;
    use ::graph::capture_node::CaptureNode;
    use ::graph::render_node::RenderNode;
    use ::graph::sample_rate_converter::SampleRateConverter;

    let mut core = Core::new().unwrap();

    let capture_node = CaptureNode::new(capture_client).unwrap();
    let render_node = RenderNode::new(render_client).unwrap();

    let input_format = capture_node.get_format();
    let output_format = render_node.get_format();

    println!("Input Format: {:?}", input_format);
    println!("Output Format: {:?}", output_format);

    let converter = SampleRateConverter::new(input_format, output_format);

    let process = render_node.send_all(
        capture_node
            //.map(move |item| converter.convert(item))
    );

    core.run(process).unwrap();
}
