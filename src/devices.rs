use std::io::Error as IoError;
use std::io::ErrorKind;
use std::mem;
use std::ptr;
use winapi::um::*;
use winapi::Interface;
use winapi::shared::wtypes::{PROPERTYKEY};
use winapi::shared::minwindef::{UINT};
use winapi::shared::guiddef::{GUID};

use ::audio_client::AudioClient;
use util::check_result;

pub enum DataFlow {
    Render,
    Capture,
    All,
}

pub enum Role {
    Console,
    Multimedia,
    Communications,
}

pub enum DeviceState {
    Active,
    Disabled,
    NotPresent,
    Unplugged,
    All,
}

pub struct Device {
    pointer: *mut mmdeviceapi::IMMDevice,
}

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
        let x = self as *const _;  // take a raw pointer to the struct
        let x = x as *const u32;    // cast the pointer from struct type to u32
        let x = *x;                 // dereference the pointer (x is now u32)
        let x = x as *const u16;    // cast the u32 to a pointer again
        from_wide_ptr(x)            // x now points to the referenced string
    }
}

impl Device {
    pub fn new(pointer: *mut mmdeviceapi::IMMDevice) -> Device {
        Device {
            pointer: pointer
        }
    }

    pub fn release(&self) {
        unsafe { (*self.pointer).Release(); }
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
            assert!(vt == 31); // assert that propertyvariant has the datatype we expect https://msdn.microsoft.com/en-us/library/windows/desktop/aa380072(v=vs.85).aspx
            let p = PropertyVariantData { data: variant.data };

            Ok(p.as_lpwstr())
        }
    }
}

pub struct DeviceCollection {
    pointer: *mut mmdeviceapi::IMMDeviceCollection,
}

impl DeviceCollection {
    pub fn new (pointer: *mut mmdeviceapi::IMMDeviceCollection) -> DeviceCollection {
        DeviceCollection {
            pointer: pointer
        }
    }

    pub fn len(&self) -> Result<usize, IoError> {
        let count = unsafe {
            let count = mem::uninitialized();
            try!(check_result((*self.pointer).GetCount(count)));
            *count
        };

        Ok(count as usize)
    }

    pub fn get_item(&self, index: usize) -> Result<Device, IoError> {
        unsafe {
            let mut device = mem::uninitialized();
            try!(check_result((*self.pointer).Item(index as UINT, &mut device)));
            Ok(Device{
                pointer: device
            })
        }
    }
}

pub struct DeviceEnumerator {
    pointer: *mut mmdeviceapi::IMMDeviceEnumerator,
}

impl DeviceEnumerator {
    pub fn new() -> Result<DeviceEnumerator, IoError> {
        let enumerator = unsafe {
            let mut enumerator: *mut mmdeviceapi::IMMDeviceEnumerator = mem::uninitialized();
            try!(check_result(combaseapi::CoCreateInstance(&mmdeviceapi::CLSID_MMDeviceEnumerator, 
                ptr::null_mut(),
                combaseapi::CLSCTX_ALL,
                &mmdeviceapi::IMMDeviceEnumerator::uuidof(),
                &mut enumerator as *mut *mut mmdeviceapi::IMMDeviceEnumerator as *mut _)));
            enumerator
        };

        Ok(DeviceEnumerator {
            pointer: enumerator,
        })
    }

    pub fn get_default_audio_endpoint(&self, data_flow: DataFlow, role: Role) -> Result<Device, IoError> {
        let e_data_flow = match data_flow {
            DataFlow::Render => mmdeviceapi::eRender,
            DataFlow::Capture => mmdeviceapi::eCapture,
            DataFlow::All => mmdeviceapi::eAll,
        };

        let e_role = match role {
            Role::Console => mmdeviceapi::eConsole,
            Role::Multimedia => mmdeviceapi::eMultimedia,
            Role::Communications => mmdeviceapi::eCommunications,
        };

        unsafe {
            if let Some(enumerator) = self.pointer.as_mut() {
                let mut device = mem::uninitialized();
                try!(check_result(enumerator.GetDefaultAudioEndpoint(e_data_flow, e_role, &mut device)));
                Ok(Device{
                    pointer: device
                })
            } else {
                Err(IoError::new(ErrorKind::Other, "the actual DeviceEnumerator is null"))
            }
        }
    }

    pub fn get_audio_endpoints(&self, data_flow: DataFlow, state: DeviceState) -> Result<DeviceCollection, IoError> {
        let e_data_flow = match data_flow {
            DataFlow::Render => mmdeviceapi::eRender,
            DataFlow::Capture => mmdeviceapi::eCapture,
            DataFlow::All => mmdeviceapi::eAll,
        };

        let e_state = match state {
            DeviceState::Active => mmdeviceapi::DEVICE_STATE_ACTIVE,
            DeviceState::Disabled => mmdeviceapi::DEVICE_STATE_ACTIVE,
            DeviceState::NotPresent => mmdeviceapi::DEVICE_STATE_ACTIVE,
            DeviceState::Unplugged => mmdeviceapi::DEVICE_STATE_ACTIVE,
            DeviceState::All => mmdeviceapi::DEVICE_STATE_ACTIVE,
        };

        unsafe {
            if let Some(enumerator) = self.pointer.as_mut() {
                let mut collection_ptr = mem::uninitialized();
                try!(check_result(enumerator.EnumAudioEndpoints(e_data_flow, e_state, &mut collection_ptr)));
                Ok(DeviceCollection::new(collection_ptr))
            } else {
                Err(IoError::new(ErrorKind::Other, "the actual DeviceEnumerator is null"))
            }
        }
    }
}
