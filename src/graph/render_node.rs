use std::io::Error as IoError;
use futures::{ Async, AsyncSink, StartSend, Poll };
use futures::sink::Sink;
use ::graph::audio_buffer::AudioBuffer;

pub struct RenderNode {

}

impl Sink for RenderNode {
    type SinkItem = AudioBuffer;
    type SinkError = IoError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        
        println!("received");

        self.poll_complete();
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl RenderNode {
    pub fn new() -> RenderNode {
        RenderNode {}
    }
}