pub mod device;
pub mod device_collection;
pub mod device_enumerator;
pub mod session;
pub mod audio_client;
pub mod audio_capture_client;
pub mod audio_render_client;

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
