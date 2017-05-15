use std::io::Error as IoError;
use ::platform::windows::{ DataFlow, Role, DeviceState };
use ::devices::{ Devices, DeviceInformation };
use ::graph_builder::{ Graph, Node };
use ::graph::capture_node::CaptureNode;
use ::graph::render_node::RenderNode;
use ::graph::sample_rate_converter::SampleRateConverter;


pub struct Router {
    devices: Devices,
}

impl Router {
    pub fn new() -> Result<Router, IoError> {
        let devices = try!(Devices::new());

        Ok(Router {
            devices: devices,
        })
    }

    pub fn get_capturing_devices(&self) -> Result<Vec<DeviceInformation>, IoError> {
        self.devices.get_devices(DataFlow::Capture)
    }

    pub fn get_default_capturing_device(&self) -> Result<DeviceInformation, IoError> {
        self.devices.get_default_device(DataFlow::Capture)
    }

    pub fn get_rendering_devices(&self) -> Result<Vec<DeviceInformation>, IoError> {
        self.devices.get_devices(DataFlow::Render)
    }

    pub fn get_default_rendering_device(&self) -> Result<DeviceInformation, IoError> {
        self.devices.get_default_device(DataFlow::Render)
    }

    pub fn run(&mut self, graph: Graph) -> Result<(), IoError> {
        use futures::Stream;
        use tokio_core::reactor::Core;
        
        let mut from = None;
        let mut to = None;
        let mut convert = None;

        for node in &graph.nodes {
            match node {
                &Node::Capture{ ref id, ref capture_device } => {
                    let device = try!(self.devices.get_device_by_id(&capture_device));
                    let audio_client = try!(device.activate());
                    let capture_node = try!(CaptureNode::new(audio_client));
                    from = Some(capture_node);
                }
                &Node::Render{ ref id, ref render_device } => {
                    let device = try!(self.devices.get_device_by_id(&render_device));
                    let audio_client = try!(device.activate());
                    let render_node = try!(RenderNode::new(audio_client));
                    to = Some(render_node);
                }
                &Node::Resample{ ref id, from, to } => {
                    let resampler = SampleRateConverter::new(from, to);
                    convert = Some(resampler);
                }
            }
        }

        let mut from = from.unwrap();
        let mut to = to.unwrap();
        let mut convert = convert.unwrap();

        let mut core = Core::new().unwrap();

        let done = from.map(|x| convert.convert(x)).forward(to);
        core.run(done).unwrap();
        Ok(())
    }
}
