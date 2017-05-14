use std::io::Error as IoError;
use futures::{ Async, AsyncSink, StartSend, Poll };
use futures::stream::Stream;
use futures::sink::Sink;
use futures::task;
use ::graph::audio_buffer::AudioBuffer;
use ::graph::audio_format::AudioFormat;

pub struct SplitNode {
    format: AudioFormat,
}

impl Stream for SplitNode {
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

    }
}

impl Sink for SplitNode {
    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {

    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl SplitNode {
    pub fn new(format: AudioFormat) -> SplitNode {
        SplitNode {
            format: format,
        }
    }
}
