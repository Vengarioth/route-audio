use std::io::Error as IoError;
use std::io::ErrorKind;
use std::mem;
use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use winapi::um::*;
use winapi::Interface;
use winapi::shared::wtypes::{PROPERTYKEY};
use winapi::shared::minwindef::{UINT};
use winapi::shared::guiddef::{GUID};
use ::platform::windows::device::Device;
use ::platform::windows::device_collection::DeviceCollection;
use ::platform::windows::{ DataFlow, Role, DeviceState };
use ::util::check_result;

#[derive(Debug)]
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

    pub fn get_device(&self, device_id: &str) -> Result<Device, IoError> {
        unsafe {
            let mut device = mem::uninitialized();
            let id_str: Vec<u16> = OsString::from(device_id).encode_wide(). chain(Some(0).into_iter()).collect();
            let id_str = id_str.as_ptr();
            try!(check_result((*self.pointer).GetDevice(id_str, &mut device)));
            Ok(Device::new(device))
        }
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
                Ok(Device::new(device))
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
