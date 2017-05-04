extern crate route_audio;

use route_audio::*;

fn main() {
    let mut session = session::Session::new().unwrap();
    let device_enumerator = session.get_device_enumerator().unwrap();

    println!("Render Devices:");
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::Render, devices::DeviceState::All).unwrap();
    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();

        match device.get_name() {
            Ok(name) => println!("  device {} \"{}\"", i, name),
            Err(e) => println!("    could not find a name property ({})", e),
        }
    }

    let mut capture_device = None;
    println!("Capture Devices:");
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::Capture, devices::DeviceState::All).unwrap();
    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();
        if i == 0 {
            capture_device = Some(device.activate().unwrap());
        }

        match device.get_name() {
            Ok(name) => println!("  device {} \"{}\"", i, name),
            Err(e) => println!("    could not find a name property ({})", e),
        }
    }

    match capture_device {
        Some(device) => run(device),
        None => println!("No capture device found."),
    }
}
