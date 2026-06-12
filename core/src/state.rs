use capture::sniff::{Sniffer};
use anyhow::Result;

struct AppState {
    pub sniffer: Sniffer,
}

impl AppState {
    pub fn new() -> Result<Self>{
        let sniffer = Sniffer::new(Some("wlan0"))?;
        Ok(AppState {
            sniffer,
        })
    }
}
