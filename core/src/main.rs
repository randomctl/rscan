use core::config::{Cli, help, usage_display, version};
use std::process::exit;

use capture::sniff::Sniffer;

fn main() {
    let args = std::env::args();
    match Cli::parse(args) {
        Ok(Cli::Help) => {
            help();
            exit(0);
        }
        Ok(Cli::Version) => {
            version();
            exit(0);
        }
        Ok(Cli::ListDevices) => {
            Sniffer::list_devices();
            exit(0);
        }
        Ok(Cli::Config(_)) => {
            todo!()
        }
        Err(err) => {
            eprintln!("{}", err);
            usage_display();
            exit(1);
        }
    }
}
