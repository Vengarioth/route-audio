use std::io::Error as IoError;
use futures::{ Async, AsyncSink, StartSend, Poll };
use futures::sink::Sink;
use ::graph::audio_buffer::AudioBuffer;
use ::graph::audio_format::AudioFormat;
use ::audio_client::{ AudioClient, AudioRenderClient };

pub struct RenderNode {
    audio_client: AudioClient,
    audio_render_client: AudioRenderClient,
    format: AudioFormat,
}

impl Sink for RenderNode {
    type SinkItem = AudioBuffer;
    type SinkError = IoError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        
        println!("renderer:");
        let frames = item.get_frames_count();
        println!("  rendering {} frames", frames);
        match self.audio_render_client.get_buffer(frames) {
            Ok(render_buffer_pointer) => {
                let raw = item.get_raw_data();
                unsafe {
                    for i in 0..raw.len() {
                        (*(render_buffer_pointer.offset(i as isize))) = raw[i];
                    }
                }
                self.audio_render_client.release_buffer(frames);
            }
            Err(e) => {
                println!("  could not allocate buffer");
            }
        }

        self.poll_complete();
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl RenderNode {
    pub fn new(audio_client: AudioClient) -> Result<RenderNode, IoError> {
        let mix_format = try!(audio_client.get_mix_format());
        let format = unsafe { AudioFormat::from_wave_format_ex((*mix_format)) };
        try!(audio_client.initialize(mix_format));
        let audio_render_client = try!(audio_client.get_render_client());
        
        try!(audio_client.start());

        Ok(RenderNode {
            audio_client: audio_client,
            audio_render_client: audio_render_client,
            format: format,
        })
    }

    pub fn get_format(&self) -> AudioFormat {
        self.format
    }
}
