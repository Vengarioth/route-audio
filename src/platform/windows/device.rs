use std::io::Error as IoError;
use std::mem;
use std::ptr;
use winapi::um::*;
use winapi::Interface;
use winapi::shared::guiddef::{GUID};
use winapi::shared::wtypes::{PROPERTYKEY};
use ole32;
use ::util::check_result;
use ::platform::windows::audio_client::AudioClient;

#[repr(C)]
#[derive(Debug)]
struct PropertyVariantData {
    data: [u8; 16]
}

fn from_wide_ptr(ptr: *const u16) -> String {
    use std::isize;
    use std::slice;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    unsafe {
        assert!(!ptr.is_null());
        let len = (0..isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
        let slice = slice::from_raw_parts(ptr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

impl PropertyVariantData {
    pub unsafe fn as_lpwstr(&self) -> String {
        let x = self as *const _;   // take a raw pointer to the struct
        let x = x as *const u32;    // cast the pointer from struct type to u32
        let x = *x;                 // dereference the pointer (x is now u32)
        let x = x as *const u16;    // cast the u32 to a pointer again
        from_wide_ptr(x)            // x now points to the referenced string
    }
}

#[derive(Debug)]
pub struct Device {
    pointer: *mut mmdeviceapi::IMMDevice,
}

impl Device {
    pub fn new(pointer: *mut mmdeviceapi::IMMDevice) -> Device {
        Device {
            pointer: pointer
        }
    }

    pub fn activate(&self) -> Result<AudioClient, IoError> {
        unsafe {
            let mut audio_client_ptr = mem::uninitialized();
            try!(check_result((*self.pointer).Activate(&audioclient::IID_IAudioClient, combaseapi::CLSCTX_ALL, ptr::null_mut(), &mut audio_client_ptr)));
            assert!(!audio_client_ptr.is_null());
            let audio_client = audio_client_ptr as *mut audioclient::IAudioClient;
            Ok(AudioClient::new(audio_client))
        }
    }

    pub fn get_id(&self) -> Result<String, IoError> {
        unsafe {
            let mut ptr = mem::uninitialized();
            try!(check_result((*self.pointer).GetId(&mut ptr)));

            let x = from_wide_ptr(ptr);

            ole32::CoTaskMemFree(ptr as *mut _);

            Ok(x)
        }
    }

    pub fn get_name(&self) -> Result<String, IoError> {
        unsafe {
            let mut propkey = PROPERTYKEY {
                fmtid: GUID {
                    Data1: 0xa45c254e,
                    Data2: 0xdf1c,
                    Data3: 0x4efd,
                    Data4: [0x80, 0x20, 0x67, 0xd1, 0x46, 0xa8, 0x50, 0xe0],
                },
                pid: 14,
            };

            let mut properties = mem::uninitialized();
            try!(check_result((*self.pointer).OpenPropertyStore(coml2api::STGM_READ, &mut properties)));

            let mut variant = mem::uninitialized();
            try!(check_result((*properties).GetValue(&mut propkey, &mut variant)));

            let vt = variant.vt;
            let p = PropertyVariantData { data: variant.data };

            Ok(p.as_lpwstr())
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { (*self.pointer).Release(); }
    }
}
