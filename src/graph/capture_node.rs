use std::io::Error as IoError;
use futures::{ Async, Poll };
use futures::stream::Stream;
use futures::task;
use ::graph::audio_buffer::AudioBuffer;
use ::audio_client::{AudioClient, AudioCaptureClient};

pub struct CaptureNode {
    audio_client: AudioClient,
    audio_capture_client: AudioCaptureClient,
    block_align: usize,
}

impl Stream for CaptureNode {
    type Item = AudioBuffer;
    type Error = IoError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut buffer = AudioBuffer::new();

        let mut next_packet_size = try!(self.audio_capture_client.get_next_packet_size());

        if next_packet_size < 1 {
            let current_task = task::park();
            // TODO more efficient polling mechanism?
            current_task.unpark();
            return Ok(Async::NotReady);
        }
        
        println!("polled, {} events ready", next_packet_size);

        while next_packet_size != 0 {
            let (frames_available, buffer_pointer) = try!(self.audio_capture_client.get_buffer());
            let bytes_to_read = self.block_align * frames_available;

            unsafe {
                for i in 0..bytes_to_read {
                    let byte = *buffer_pointer.offset(i as isize);
                    buffer.push(byte);
                }
            }

            try!(self.audio_capture_client.release_buffer(frames_available));
            next_packet_size = try!(self.audio_capture_client.get_next_packet_size());
        }

        Ok(Async::Ready(Some(buffer)))
    }
}

impl CaptureNode {
    pub fn new(audio_client: AudioClient) -> Result<CaptureNode, IoError> {
        let mix_format = try!(audio_client.get_mix_format());
        let block_align = unsafe { (*mix_format).nBlockAlign as usize };
        try!(audio_client.initialize(mix_format));
        let audio_capture_client = try!(audio_client.get_capture_client());

        try!(audio_client.start());

        Ok(CaptureNode {
            audio_client: audio_client,
            audio_capture_client: audio_capture_client,
            block_align: block_align,
        })
    }
}
