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

    println!("Capture Devices:");
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::Capture, devices::DeviceState::All).unwrap();
    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();

        match device.get_name() {
            Ok(name) => println!("  device {} \"{}\"", i, name),
            Err(e) => println!("    could not find a name property ({})", e),
        }
    }

    let capture_device = device_enumerator.get_default_audio_endpoint(devices::DataFlow::Capture, devices::Role::Console).unwrap();
    let render_device = device_enumerator.get_default_audio_endpoint(devices::DataFlow::Render, devices::Role::Console).unwrap();

    match capture_device.get_name() {
        Ok(name) => println!("using capture device \"{}\"", name),
        Err(e) => println!("    could not find a name property ({})", e),
    }
    match render_device.get_name() {
        Ok(name) => println!("using render device \"{}\"", name),
        Err(e) => println!("    could not find a name property ({})", e),
    }

    run(capture_device.activate().unwrap(), render_device.activate().unwrap());
}
