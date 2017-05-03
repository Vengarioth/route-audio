extern crate route_audio;

use route_audio::*;

fn main() {
    let mut session = session::Session::new().unwrap();
    let device_enumerator = session.get_device_enumerator().unwrap();
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::All, devices::DeviceState::All).unwrap();

    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();

        match device.get_name() {
            Ok(name) => println!("device \"{}\"", name),
            Err(e) => println!("could not find a name property ({})", e),
        }
    }
}
