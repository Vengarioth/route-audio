pub enum WaveFormat {
    Mpeg,
    MpegLayer3,
    Extensible
}

pub struct AudioFormat {
    pub format: WaveFormat,
    pub channels: u32,
    pub samples_per_second: u32,
    pub average_bytes_per_second: u32,
    pub block_align: u32,
    pub bits_per_sample: u32
}

impl AudioFormat {
    pub fn new(format: WaveFormat, channels: u32, samples_per_second: u32, average_bytes_per_second: u32, block_align: u32, bits_per_sample: u32) -> AudioFormat {
        AudioFormat {
            format: format,
            channels: channels,
            samples_per_second: samples_per_second,
            average_bytes_per_second: average_bytes_per_second,
            block_align: block_align,
            bits_per_sample: bits_per_sample,
        }
    }
}
