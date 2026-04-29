build:
    cargo build
    cargo install --path core --bin rscan --force

test: build
    cargo test

unit-all: build
    cargo test --lib

pktest target: build
    cargo test --package {{target}}

