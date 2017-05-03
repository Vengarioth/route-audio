extern crate winapi;
extern crate kernel32;

use std::io::Error as IoError;
use winapi::ctypes::*;
use winapi::um::*;
use winapi::shared::*;
use winapi::shared::basetsd::{UINT32};
use std::mem;
use std::ptr;
use std::thread;
use winapi::um::winnt::HRESULT;
use winapi::um::strmif::REFERENCE_TIME;
use winapi::Interface;

const REFTIMES_PER_SEC: REFERENCE_TIME = 10000000;
const REFTIMES_PER_MILLISEC: REFERENCE_TIME = 10000;

fn check_result(result: HRESULT) -> Result<(), IoError> {
    if result < 0 {
        unsafe {
            let last_error = errhandlingapi::GetLastError();
            println!("last error: {}", last_error);
        }
        Err(IoError::from_raw_os_error(result))
    } else {
        Ok(())
    }
}

fn main() {
    unsafe {
        check_result(combaseapi::CoInitializeEx(ptr::null_mut(), objbase::COINIT_MULTITHREADED)).unwrap();

        let mut enumerator: *mut mmdeviceapi::IMMDeviceEnumerator = mem::uninitialized();
        let hresult = combaseapi::CoCreateInstance(&mmdeviceapi::CLSID_MMDeviceEnumerator, 
            ptr::null_mut(),
            combaseapi::CLSCTX_ALL,
            &mmdeviceapi::IMMDeviceEnumerator::uuidof(),
            &mut enumerator as *mut *mut mmdeviceapi::IMMDeviceEnumerator as *mut _);
        check_result(hresult).unwrap();

        if let Some(enumerator) = enumerator.as_mut() {
            
            let mut device = mem::uninitialized();
            check_result(enumerator.GetDefaultAudioEndpoint(mmdeviceapi::eCapture, mmdeviceapi::eConsole, &mut device)).unwrap();

            let mut state = mem::uninitialized();
            check_result((*device).GetState(&mut state)).unwrap();

            let state_name = match state {
                mmdeviceapi::DEVICE_STATE_ACTIVE => "active",
                mmdeviceapi::DEVICE_STATE_DISABLED => "disabled",
                mmdeviceapi::DEVICE_STATE_NOTPRESENT => "not present",
                mmdeviceapi::DEVICE_STATE_UNPLUGGED => "unplugged",
                _ => "unknown"
            };

            println!("device state {}", state_name);

            let mut audio_client_ptr = mem::uninitialized();
            let result = (*device).Activate(&audioclient::IID_IAudioClient, combaseapi::CLSCTX_ALL, ptr::null_mut(), &mut audio_client_ptr);
            assert!(!audio_client_ptr.is_null());
            let audio_client = audio_client_ptr as *mut audioclient::IAudioClient;

            let mut format_ptr = mem::uninitialized();
            check_result((*audio_client).GetMixFormat(&mut format_ptr)).unwrap();

            let share_mode = audiosessiontypes::AUDCLNT_SHAREMODE_SHARED;
            check_result((*audio_client).Initialize(share_mode, 0, REFTIMES_PER_SEC, 0, format_ptr, ptr::null())).unwrap();
            println!("wFormatTag {}", (*format_ptr).wFormatTag);
            println!("nChannels {}", (*format_ptr).nChannels);
            println!("nSamplesPerSec {}", (*format_ptr).nSamplesPerSec);
            println!("nAvgBytesPerSec {}", (*format_ptr).nAvgBytesPerSec);
            println!("nBlockAlign {}", (*format_ptr).nBlockAlign);
            println!("wBitsPerSample {}", (*format_ptr).wBitsPerSample);
            println!("cbSize {}", (*format_ptr).cbSize);

            let mut max_frames_in_buffer = mem::uninitialized();
            check_result((*audio_client).GetBufferSize(&mut max_frames_in_buffer)).unwrap();

            let mut capture_client_ptr = mem::uninitialized();
            check_result((*audio_client).GetService(&audioclient::IID_IAudioCaptureClient, &mut capture_client_ptr)).unwrap();
            assert!(!capture_client_ptr.is_null());
            let capture_client = capture_client_ptr as *mut audioclient::IAudioCaptureClient;

            //set format on sink

            let duration = (REFTIMES_PER_SEC as f64 * max_frames_in_buffer as f64) / (*format_ptr).nSamplesPerSec as f64;
            println!("buffer duration: {}ms", duration);

            check_result((*audio_client).Start()).unwrap();

            let mut data = mem::uninitialized();
            let mut frames_available = mem::uninitialized();
            let mut flags = mem::uninitialized();
            let mut packet_length: UINT32 = mem::uninitialized();
            

            loop {
                println!("sleep for {}", (duration / REFTIMES_PER_MILLISEC as f64 / 2.0) as u32);
                thread::sleep_ms((duration / REFTIMES_PER_MILLISEC as f64 / 2.0) as u32);
                check_result((*capture_client).GetNextPacketSize(&mut packet_length)).unwrap();
                println!("packet_length {}", packet_length);

                while packet_length != 0 {
                    check_result((*capture_client).GetBuffer(&mut data, &mut frames_available, &mut flags, ptr::null_mut(), ptr::null_mut())).unwrap();
                    println!("frames_available {}", frames_available);
                    println!("flags {}", flags);

                    check_result((*capture_client).ReleaseBuffer(frames_available)).unwrap();
                    check_result((*capture_client).GetNextPacketSize(&mut packet_length)).unwrap();
                    println!("packet_length {}", packet_length);
                }
            }

            check_result((*audio_client).Stop()).unwrap();
            combaseapi::CoTaskMemFree(format_ptr as *mut c_void);
            (*enumerator).Release();
            (*device).Release();
            (*audio_client).Release();
            (*capture_client).Release();
        }
    }
}
