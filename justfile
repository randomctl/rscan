build:
    cargo build
    cargo install --path core --bin rscan --force

test: build
    cargo test

unit: build
    cargo test --lib

unit-all: build
    cargo test --lib -- --include-ignored

pktest target: build
    cargo test --package {{target}}

cov:
    LLVM_COV="$(rustup run stable rustc --print target-libdir)/../bin/llvm-cov" \
    LLVM_PROFDATA="$(rustup run stable rustc --print target-libdir)/../bin/llvm-profdata" \
    cargo +stable llvm-cov --workspace

cov-check min="90":
    ./scripts/cov-check.sh {{min}}
