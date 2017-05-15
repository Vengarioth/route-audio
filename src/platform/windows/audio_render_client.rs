use std::io::Error as IoError;
use winapi::um::*;
use winapi::um::strmif::REFERENCE_TIME;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::shared::basetsd::{UINT32};
use winapi::shared::minwindef::{BYTE};
use std::mem;
use std::ptr;
use ::util::check_result;

#[derive(Debug)]
pub struct AudioRenderClient {
    pointer: *mut audioclient::IAudioRenderClient,
}

impl AudioRenderClient {
    pub fn new(pointer: *mut audioclient::IAudioRenderClient) -> AudioRenderClient {
        AudioRenderClient {
            pointer: pointer,
        }
    }

    pub fn get_buffer(&self, frames_requested: usize) -> Result<*mut BYTE, IoError> {
        unsafe {
            let mut data = mem::uninitialized();
            try!(check_result((*self.pointer).GetBuffer(frames_requested as UINT32, &mut data)));
            Ok(data)
        }
    }

    pub fn release_buffer(&self, frames_written: usize) -> Result<(), IoError> {
        unsafe {
            try!(check_result((*self.pointer).ReleaseBuffer(frames_written as UINT32, 0)));
            Ok(())
        }
    }
}
