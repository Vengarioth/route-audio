use std::io::Error as IoError;
use std::io::ErrorKind;
use std::ptr;
use winapi::um::*;

use util::check_result;
use devices::DeviceEnumerator;

pub struct Session {
    initialized: bool,
}

impl Session {
    pub fn new() -> Result<Session, IoError> {
        unsafe {
            try!(check_result(combaseapi::CoInitializeEx(ptr::null_mut(), objbase::COINIT_MULTITHREADED)));
        }
        Ok(Session {
            initialized: true,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.initialized
    }

    pub fn get_device_enumerator(&mut self) -> Result<DeviceEnumerator, IoError> {
        if !self.initialized {
            return Err(IoError::new(ErrorKind::Other, "Session not or no longer initialized."));
        }

        let device_enumerator = try!(DeviceEnumerator::new());

        Ok(device_enumerator)
    }
}
