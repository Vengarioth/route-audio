use std::io::Error as IoError;
use winapi::um::winnt::HRESULT;

pub fn check_result(result: HRESULT) -> Result<(), IoError> {
    if result < 0 {
        Err(IoError::from_raw_os_error(result))
    } else {
        Ok(())
    }
}
