use std::io::Error as IoError;
use std::io::ErrorKind;
use std::mem;
use std::ptr;
use winapi::um::*;
use winapi::Interface;
use winapi::shared::wtypes::{PROPERTYKEY};
use winapi::shared::minwindef::{UINT};
use winapi::shared::guiddef::{GUID};
use ::platform::windows::device::Device;
use ::util::check_result;

#[derive(Debug)]
pub struct DeviceCollection {
    pointer: *mut mmdeviceapi::IMMDeviceCollection,
}

impl DeviceCollection {
    pub fn new(pointer: *mut mmdeviceapi::IMMDeviceCollection) -> DeviceCollection {
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
            Ok(Device::new(device))
        }
    }
}
