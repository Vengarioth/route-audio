use ::devices::DeviceInformation;
use ::graph::audio_format::AudioFormat;

#[derive(Debug)]
pub enum Node {
    Capture { id: u32, capture_device: String },
    Render { id: u32, render_device: String },
    Resample { id: u32, from: AudioFormat, to: AudioFormat },
}

#[derive(Debug, Copy, Clone)]
pub struct NodeReference {
    id: u32,
}

#[derive(Debug)]
pub struct Connection {
    pub input_node: u32,
    pub output_node: u32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

#[derive(Debug)]
pub struct GraphBuilder {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    id_iterator: u32,
}

impl GraphBuilder {

    pub fn new() -> GraphBuilder {
        GraphBuilder {
            nodes: Vec::new(),
            connections: Vec::new(),
            id_iterator: 0,
        }
    }

    pub fn add_capture_node(&mut self, device: &DeviceInformation) -> NodeReference {
        let id = self.get_next_id();
        self.nodes.push(Node::Capture {
            id: id.clone(),
            capture_device: device.id.clone(),
        });
        NodeReference { id: id }
    }

    pub fn add_resample_node(&mut self, from_format: AudioFormat, to_format: AudioFormat) -> NodeReference {
        let id = self.get_next_id();
        self.nodes.push(Node::Resample {
            id: id.clone(),
            from: from_format,
            to: to_format,
        });
        NodeReference { id: id }
    }

    pub fn add_render_node(&mut self, device: &DeviceInformation) -> NodeReference {
        let id = self.get_next_id();
        self.nodes.push(Node::Render {
            id: id.clone(),
            render_device: device.id.clone(),
        });
        NodeReference { id: id }
    }

    pub fn connect(&mut self, from: &NodeReference, to: &NodeReference) {
        self.connections.push(Connection {
            input_node: from.id,
            output_node: to.id
        });
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.id_iterator.clone();
        self.id_iterator += 1;
        id
    }

    pub fn build(self) -> Graph {
        Graph {
            nodes: self.nodes,
            connections: self.connections,
        }
    }

}