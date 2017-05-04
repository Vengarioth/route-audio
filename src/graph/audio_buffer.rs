#[derive(Debug)]
pub struct AudioBuffer {
    buffer: Vec<u8>
}

impl AudioBuffer {
    pub fn new() -> AudioBuffer {
        AudioBuffer {
            buffer: Vec::new()
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.buffer.push(byte);
    }
}
