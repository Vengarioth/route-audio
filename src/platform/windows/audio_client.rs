use std::io::Error as IoError;
use winapi::um::*;
use winapi::um::strmif::REFERENCE_TIME;
use winapi::shared::mmreg::WAVEFORMATEX;
use winapi::shared::basetsd::{UINT32};
use winapi::shared::minwindef::{BYTE};
use std::mem;
use std::ptr;
use ::util::check_result;
use ::platform::windows::audio_capture_client::AudioCaptureClient;
use ::platform::windows::audio_render_client::AudioRenderClient;

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

    pub fn get_render_client(&self) -> Result<AudioRenderClient, IoError> {
        unsafe {
            let mut render_client_ptr = mem::uninitialized();
            try!(check_result((*self.pointer).GetService(&audioclient::IID_IAudioRenderClient, &mut render_client_ptr)));
            assert!(!render_client_ptr.is_null());
            let render_client = render_client_ptr as *mut audioclient::IAudioRenderClient;
            Ok(AudioRenderClient::new(render_client))
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
