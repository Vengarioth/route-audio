extern crate route_audio;

use route_audio::*;

fn main() {
    let mut router = router::Router::new().unwrap();
    let capturing_devices = router.get_capturing_devices().unwrap();
    println!("capturing devices:");
    for capturing_device in capturing_devices {
        println!("  {:?}", capturing_device);
    }

    let rendering_devices = router.get_rendering_devices().unwrap();
    println!("rendering devices:");
    for rendering_device in rendering_devices {
        println!("  {:?}", rendering_device);
    }

    let capturing_device = router.get_default_capturing_device().unwrap();
    println!("using capturing device: {:?}", capturing_device);
    let rendering_device = router.get_default_rendering_device().unwrap();
    println!("using rendering device: {:?}", rendering_device);

    let mut graph_builder = graph_builder::GraphBuilder::new();
    let capture_node = graph_builder.add_capture_node(&capturing_device);
    let render_node = graph_builder.add_render_node(&rendering_device);
    let resample_node = graph_builder.add_resample_node();
    graph_builder.connect(&capture_node, &resample_node);
    graph_builder.connect(&resample_node, &render_node);

    let graph = graph_builder.build();

    router.run(graph);
}
