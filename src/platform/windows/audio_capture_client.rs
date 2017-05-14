use std::io::Error as IoError;
use winapi::um::*;
use winapi::um::strmif::REFERENCE_TIME;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::shared::basetsd::{UINT32};
use winapi::shared::minwindef::{BYTE};
use std::mem;
use std::ptr;
use ::util::check_result;

pub struct AudioCaptureClient {
    pointer: *mut audioclient::IAudioCaptureClient,
}

impl AudioCaptureClient {
    pub fn new(pointer: *mut audioclient::IAudioCaptureClient) -> AudioCaptureClient {
        AudioCaptureClient {
            pointer: pointer,
        }
    }

    pub fn get_next_packet_size(&self) -> Result<usize, IoError> {
        unsafe {
            let mut packet_length: UINT32 = mem::uninitialized();
            try!(check_result((*self.pointer).GetNextPacketSize(&mut packet_length)));
            Ok(packet_length as usize)
        }
    }

    pub fn get_buffer(&self) -> Result<(usize, *const BYTE), IoError> {
        unsafe {
            let mut data = mem::uninitialized();
            let mut frames_available = mem::uninitialized();
            let mut flags = mem::uninitialized();
            try!(check_result((*self.pointer).GetBuffer(&mut data, &mut frames_available, &mut flags, ptr::null_mut(), ptr::null_mut())));
            Ok((frames_available as usize, data))
        }
    }

    pub fn release_buffer(&self, frames_written: usize) -> Result<(), IoError> {
        unsafe {
            try!(check_result((*self.pointer).ReleaseBuffer(frames_written as UINT32)));
            Ok(())
        }
    }
}
