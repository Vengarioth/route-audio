extern crate route_audio;

use route_audio::*;

fn main() {
    let mut session = session::Session::new().unwrap();
    let device_enumerator = session.get_device_enumerator().unwrap();

    println!("Render Devices:");
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::Render, devices::DeviceState::All).unwrap();
    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();
        let audio_client = device.activate().unwrap();
        let format = unsafe{ graph::audio_format::AudioFormat::from_wave_format_ex(*audio_client.get_mix_format().unwrap()) };

        match device.get_name() {
            Ok(name) => {
                println!("  device {} \"{}\"", i, name);
                println!("      channels: {}", format.channels);
                println!("      samples_per_second: {}", format.samples_per_second);
                println!("      average_bytes_per_second: {}", format.average_bytes_per_second);
                println!("      block_align: {}", format.block_align);
                println!("      bits_per_sample: {}", format.bits_per_sample);
            }
            Err(e) => println!("    could not find a name property ({})", e),
        }
    }

    println!("Capture Devices:");
    let devices = device_enumerator.get_audio_endpoints(devices::DataFlow::Capture, devices::DeviceState::All).unwrap();
    for i in 0..devices.len().unwrap() {
        let device = devices.get_item(i).unwrap();
        let audio_client = device.activate().unwrap();
        let format = unsafe{ graph::audio_format::AudioFormat::from_wave_format_ex(*audio_client.get_mix_format().unwrap()) };

        match device.get_name() {
            Ok(name) => {
                println!("  device {} \"{}\"", i, name);
                println!("      channels: {}", format.channels);
                println!("      samples_per_second: {}", format.samples_per_second);
                println!("      average_bytes_per_second: {}", format.average_bytes_per_second);
                println!("      block_align: {}", format.block_align);
                println!("      bits_per_sample: {}", format.bits_per_sample);
            }
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
