use std::io::Error as IoError;
use ::platform::windows::session::Session;
use ::platform::windows::device_enumerator::DeviceEnumerator;
use ::platform::windows::device::Device;
use ::platform::windows::{ DataFlow, Role, DeviceState };
use ::graph::audio_format::AudioFormat;

#[derive(Debug)]
pub struct DeviceInformation {
    pub name: String,
    pub id: String,
    pub audio_format: AudioFormat,
}

pub struct Devices {
    session: Session,
    device_enumerator: DeviceEnumerator,
}

impl Devices {

    pub fn new() -> Result<Devices, IoError> {
        let session = try!(Session::new());
        let device_enumerator = try!(session.get_device_enumerator());
        Ok(Devices {
            session: session,
            device_enumerator: device_enumerator,
        })
    }

    pub fn get_device_by_id(&self, device_id: &str) -> Result<Device, IoError> {
        self.device_enumerator.get_device(device_id)
    }

    pub fn get_devices(&self, data_flow: DataFlow) -> Result<Vec<DeviceInformation>, IoError> {
        let device_collection = try!(self.device_enumerator.get_audio_endpoints(data_flow, DeviceState::All));
        let len = try!(device_collection.len());

        let mut devices = Vec::new();
        for i in 0..len {
            let native_device = try!(device_collection.get_item(i));
            let audio_client = try!(native_device.activate());
            let format = AudioFormat::from_wave_format_ex(try!(audio_client.get_mix_format()));
            let id = try!(native_device.get_id());
            let name = try!(native_device.get_name());
            devices.push(DeviceInformation {
                id: id,
                name: name,
                audio_format: format,
            });
        }

        Ok(devices)
    }

    pub fn get_default_device(&self, data_flow: DataFlow) -> Result<DeviceInformation, IoError> {
        let native_device = try!(self.device_enumerator.get_default_audio_endpoint(data_flow, Role::Console));
        let audio_client = try!(native_device.activate());
        let format = AudioFormat::from_wave_format_ex(try!(audio_client.get_mix_format()));
        let id = try!(native_device.get_id());
        let name = try!(native_device.get_name());

        Ok(DeviceInformation {
            id: id,
            name: name,
            audio_format: format,
        })
    }
}
