# rscan

Network security scanning tool written in Rust.

The goal is to build an efficient network scanner for local LAN environments with:
- host and service discovery
- packet inspection
- built-in threat detection
- terminal user interface (TUI)

This project is currently being reorganized into a crate-based architecture.

## Crates

- `core`: orchestration/runtime integration, shared models/events, config and security checks
- `capture`: packet capture and frame decode/parsing adapters
- `probe`: active discovery and probing workflow (host/port scan pipeline)
- `detect`: detection engine and rule evaluation
- `tui`: terminal UI surface and input handling

## Usage

```bash
sudo cargo run --bin rscan -- sniff
```
