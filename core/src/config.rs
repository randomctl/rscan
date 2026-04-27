pub enum Mode {
    TUI,
    SNIFF,
}

pub fn parse_command() {
    // TODO: implement cli args parse method
    todo!()
}

fn sniff_intro() {
    let banner = r#"
 ____  ____   ____    _    _   _
|  _ \/ ___| / ___|  / \  | \ | |
| |_) \___ \| |     / _ \ |  \| |
|  _ < ___) | |___ / ___ \| |\  |
|_| \_\____/ \____/_/   \_\_| \_|
"#;
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal and position cursor at 1,1
    println!("\x1b[36m{}\x1b[0m", banner); // cyan
    println!("RSCAN v0.1.0 — Network Recon CLI\n");
}

fn help() {
    println!(
        "usage: rscan OPTION [FLAGS]

OPTIONS:
    - sniff: Passive mode. Displaying packets metadata
    - tui: Terminal User Interface mode. This is the default mode.

FLAGS:
    - -v or --verbose: for verbose output logging"
    );
}
