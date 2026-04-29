build:
    cargo build
    cargo install --path core --bin rscan --force
test: build
    cargo test


