#[derive(Debug)]
pub enum Node {
    Noop { id: u32 },
    Capture { id: u32, capture_device: String },
    Render { id: u32, render_device: String },
    Resample { id: u32, from_channels: u32, to_channels: u32, from_hertz: u32, to_herz: u32 },
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

pub fn construct(graph: Graph) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
