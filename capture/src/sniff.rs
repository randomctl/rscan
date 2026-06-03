use crate::errors::{Error, ErrorType};
use pcap::Device;

#[derive(Debug)]
pub struct Sniffer {
    device: Device,
}

impl Sniffer {
    // TODO: Manage errors in a better way.
    pub fn new(device_name: Option<&str>) -> Result<Self, Error> {
        let device: Device = match device_name {
            Some(name) => {
                let devices = Device::list().map_err(|_| {
                    Error::err(
                        ErrorType::DeviceError,
                        String::from("Unable to list devices from pcap."),
                    )
                })?;

                let device = devices
                    .into_iter()
                    .find(|device| device.name == name)
                    .ok_or_else(|| {
                        Error::err(
                            ErrorType::DeviceError,
                            format!("No device found with device name {}", name),
                        )
                    })?;

                device
            }
            None => {
                match Device::lookup().map_err(|_| {
                    Error::err(
                        ErrorType::DeviceError,
                        String::from("Unable to lookup for devices using pcap."),
                    )
                })? {
                    Some(device) => device,
                    None => {
                        return Err(Error::err(
                            ErrorType::DeviceError,
                            String::from("No default device found."),
                        ));
                    }
                }
            }
        };
        Ok(Sniffer { device: device })
    }

    pub fn next() {}
    pub fn capture_loop() {}

    pub fn list_devices() {
        match Device::list() {
            Ok(devices) => {
                for device in devices {
                    if !device.addresses.is_empty() {
                        print!("Device: {}", device.name);
                        for addr in device.addresses {
                            print!(" addr: {}", addr.addr);
                        }
                        print!("\n");
                    }
                }
            }
            Err(_) => {
                println!("No devices found.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sniffer_with_device_name_success() {
        let device_name = "wlan0";
        let sniffer = Sniffer::new(Some(device_name));
        assert_eq!(sniffer.is_ok(), true);
    }
}
