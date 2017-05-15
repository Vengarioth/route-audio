use winapi::shared::mmreg::{WAVEFORMATEX, WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_MPEG, WAVE_FORMAT_MPEGLAYER3};

#[derive(Debug, Copy, Clone)]
pub enum WaveFormat {
    Mpeg,
    MpegLayer3,
    Extensible,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

    pub fn from_wave_format_ex(format: *const WAVEFORMATEX) -> AudioFormat {
        let format = unsafe { *format };
        AudioFormat {
            format: match format.wFormatTag {
                WAVE_FORMAT_EXTENSIBLE => WaveFormat::Extensible,
                WAVE_FORMAT_MPEG => WaveFormat::Mpeg,
                WAVE_FORMAT_MPEGLAYER3 => WaveFormat::MpegLayer3,
                _ => WaveFormat::Unknown,
            },
            channels: format.nChannels as u32,
            samples_per_second: format.nSamplesPerSec,
            average_bytes_per_second: format.nAvgBytesPerSec,
            block_align: format.nBlockAlign as u32,
            bits_per_sample: format.wBitsPerSample as u32,
        }
    }
}
