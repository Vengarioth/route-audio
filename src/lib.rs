extern crate winapi;
extern crate kernel32;

pub mod devices;
pub mod session;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

    }
}
