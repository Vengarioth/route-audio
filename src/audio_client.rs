use std::io::Error as IoError;
use winapi::um::*;
use winapi::um::strmif::REFERENCE_TIME;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::shared::basetsd::{UINT32};
use winapi::shared::minwindef::{BYTE};
use std::mem;
use std::ptr;

use ::util::check_result;

const REFTIMES_PER_SEC: REFERENCE_TIME = 10000000;
const REFTIMES_PER_MILLISEC: REFERENCE_TIME = 10000;

pub struct AudioClient {
    pointer: *mut audioclient::IAudioClient,
}

impl AudioClient {
    pub fn new(pointer: *mut audioclient::IAudioClient) -> AudioClient {
        AudioClient {
            pointer: pointer
        }
    }

    pub fn start(&self) -> Result<(), IoError> {
        unsafe {
            try!(check_result((*self.pointer).Start()));
            Ok(())
        }
    }

    pub fn get_mix_format(&self) -> Result<*const WAVEFORMATEX, IoError> {
        unsafe {
            let mut format_ptr = mem::uninitialized();
            try!(check_result((*self.pointer).GetMixFormat(&mut format_ptr)));
            Ok(format_ptr)
        }
    }

    pub fn initialize(&self, format: *const WAVEFORMATEX) -> Result<(), IoError> {
        unsafe {
            try!(check_result((*self.pointer).Initialize(audiosessiontypes::AUDCLNT_SHAREMODE_SHARED, 0, REFTIMES_PER_SEC, 0, format, ptr::null())));
        }

        Ok(())
    }

    pub fn get_buffer_size(&self) -> Result<usize, IoError> {
        unsafe {
            let mut buffer_size = mem::uninitialized();
            try!(check_result((*self.pointer).GetBufferSize(&mut buffer_size)));
            Ok(buffer_size as usize)
        }
    }

    pub fn get_capture_client(&self) -> Result<AudioCaptureClient, IoError> {
        unsafe {
            let mut capture_client_ptr = mem::uninitialized();
            try!(check_result((*self.pointer).GetService(&audioclient::IID_IAudioCaptureClient, &mut capture_client_ptr)));
            assert!(!capture_client_ptr.is_null());
            let capture_client = capture_client_ptr as *mut audioclient::IAudioCaptureClient;
            Ok(AudioCaptureClient::new(capture_client))
        }
    }
}

pub struct AudioCaptureClient {
    pointer: *mut audioclient::IAudioCaptureClient,
}

impl AudioCaptureClient {
    pub fn new(pointer: *mut audioclient::IAudioCaptureClient) -> AudioCaptureClient {
        AudioCaptureClient {
            pointer: pointer
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
