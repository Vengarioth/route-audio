use ::graph::audio_format::AudioFormat;

#[derive(Debug)]
pub struct AudioBuffer {
    buffer: Vec<u8>,
    format: AudioFormat,
}

impl AudioBuffer {
    pub fn new(format: AudioFormat) -> AudioBuffer {
        AudioBuffer {
            buffer: Vec::new(),
            format: format,
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    pub fn get_raw_data(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_frames_count(&self) -> usize {
        self.buffer.len() / self.format.block_align as usize
    }
}
