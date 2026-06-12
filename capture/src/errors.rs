use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CaptureError {
    #[error("no network device found")]
    NoDeviceFound,

    #[error("error fetching available devices from pcap")]
    PcapError,
}
