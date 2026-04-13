# rscan

Network scanning tool written in rust. The goal is to make an effective tool to scan a network against intruders.
Currently doing some network research on how to achieve this.

## Crates

- `core`: packet parsing primitives (Ethernet, IPv4/IPv6, TCP/UDP)
- `runtime`: capture loop and runtime wiring
- `cli`: command-line entry point

## Usage

```bash
cargo run -p rscan -- sniff
```
