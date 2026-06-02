use pcap::Device;

pub struct Sniffer {}

impl Sniffer {
    pub fn new() -> Self {
        Sniffer {}
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
