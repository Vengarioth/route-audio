use std::io::Error as IoError;
use ::platform::windows::{ DataFlow, Role, DeviceState };
use ::devices::{ Devices, DeviceInformation };
use ::graph_builder::{ Graph, Node };
use ::graph::capture_node::CaptureNode;

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
        
        for node in &graph.nodes {
            match node {
                &Node::Capture{ ref id, ref capture_device } => {
                    let device = try!(self.devices.get_device_by_id(&capture_device));
                    let audio_client = try!(device.activate());
                    let capture_node = try!(CaptureNode::new(audio_client));
                }
                &Node::Render{ ref id, ref render_device } => {
                    // let render_node = RenderNode::new(render_client).unwrap();
                }
                &Node::Resample{ ref id, ref from_channels, ref to_channels, ref from_hertz, ref to_hertz } => {
                    // let resampler = SampleRateConverter::new(input_format, output_format);
                }
            }
        }

        Ok(())
    }
}
