use std::io::Error as IoError;
use futures::{ Async, Poll };
use futures::stream::Stream;
use futures::task;
use ::graph::audio_buffer::AudioBuffer;
use ::graph::audio_format::AudioFormat;
use ::platform::windows::audio_client::AudioClient;
use ::platform::windows::audio_capture_client::AudioCaptureClient;

pub struct CaptureNode {
    audio_client: AudioClient,
    audio_capture_client: AudioCaptureClient,
    format: AudioFormat,
}

impl Stream for CaptureNode {
    type Item = AudioBuffer;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut buffer = AudioBuffer::new(self.format);

        let mut next_packet_size = try!(self.audio_capture_client.get_next_packet_size());

        if next_packet_size < 1 {
            let current_task = task::park();
            // TODO more efficient polling mechanism?
            current_task.unpark();
            return Ok(Async::NotReady);
        }
        
        println!("capturer:");
        while next_packet_size != 0 {
            let (frames_available, buffer_pointer) = try!(self.audio_capture_client.get_buffer());
            let bytes_to_read = self.format.block_align as usize * frames_available;

            unsafe {
                for i in 0..bytes_to_read {
                    let byte = *buffer_pointer.offset(i as isize);
                    buffer.push(byte);
                }
            }

            try!(self.audio_capture_client.release_buffer(frames_available));
            next_packet_size = try!(self.audio_capture_client.get_next_packet_size());
        }
        println!("  {} frames captured", buffer.get_frames_count());

        Ok(Async::Ready(Some(buffer)))
    }
}

impl CaptureNode {
    pub fn new(audio_client: AudioClient) -> Result<CaptureNode, IoError> {
        let mix_format = try!(audio_client.get_mix_format());
        let format = unsafe { AudioFormat::from_wave_format_ex((*mix_format)) };
        try!(audio_client.initialize(mix_format));
        let audio_capture_client = try!(audio_client.get_capture_client());

        try!(audio_client.start());

        Ok(CaptureNode {
            audio_client: audio_client,
            audio_capture_client: audio_capture_client,
            format: format,
        })
    }

    pub fn get_format(&self) -> AudioFormat {
        self.format
    }
}
