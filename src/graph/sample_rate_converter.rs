use sample::interpolate::{Linear, Sinc, Floor, Converter};
use sample::Sample;
use sample::Frame;
use ::graph::audio_format::AudioFormat;
use ::graph::audio_buffer::AudioBuffer;

pub struct SampleRateConverter {
    source_format: AudioFormat,
    target_format: AudioFormat,
}

impl SampleRateConverter {
    pub fn new(source_format: AudioFormat, target_format: AudioFormat) -> SampleRateConverter {
        SampleRateConverter {
            source_format: source_format,
            target_format: target_format,
        }
    }

    pub fn convert(&self, buffer: AudioBuffer) -> AudioBuffer {
        use std::mem::transmute;

        let frames_to_convert = buffer.get_frames_count();

        println!("converter:");
        println!("  {} frames to convert", buffer.get_frames_count());
        let raw = buffer.get_raw_data().to_vec();

        let mut samples: Vec<[u8; 4]> = Vec::new();
        for i in 0..raw.len() / 4 {
            samples.push([
                raw[(i * 4)],
                raw[(i * 4) + 1],
                raw[(i * 4) + 2],
                raw[(i * 4) + 3],
            ]);
        }
        let x = unsafe { transmute::<Vec<[u8; 4]>, Vec<u32>>(samples) };
        let x: Vec<[f32; 1]> = x.iter().cloned().map(|x| [x as f32]).collect();

        let from_hz = self.source_format.samples_per_second as f64;
        let from_channels = self.source_format.channels;
        let to_hz = self.target_format.samples_per_second as f64;
        let to_channels = self.target_format.channels;
        println!("  converting from {}Hz {}channels to {}Hz {}channels", from_hz, from_channels, to_hz, to_channels);

        //let interpolator: Sinc<[f32; 1]> = Sinc::zero_padded(50);
        let interpolator: Floor<[f32; 1]> = Floor::new([0.0]);
        let conv = Converter::from_hz_to_hz(x.iter().cloned(), interpolator, from_hz, to_hz);
        let mut buffer = AudioBuffer::new(self.target_format);

        let mut y = Vec::new();
        for f in conv {
            let x = [f[0] as u32];
            let x = unsafe{ transmute::<[u32; 1], [u8; 4]>(x) };
            y.push(x);
            y.push(x);
        }
        println!("  {} converted frames", y.len() / 2);

        for x in y {
            buffer.push(x[0]);
            buffer.push(x[1]);
            buffer.push(x[2]);
            buffer.push(x[3]);
        }

        buffer
    }
}
