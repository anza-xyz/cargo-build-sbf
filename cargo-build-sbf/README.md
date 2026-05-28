# cargo-build-sbf

This repository holds the code for both `cargo-build-sbf` and `cargo-test-sbf` tools used to
build Rust programs for Solana.

## Installation

We bundled `cargo-build-sbf` and `cargo-test-sbf` in the same package, so their installation
is only a single command:

```
cargo install cargo-build-sbf
```

## Usage

Inside a Solana Rust project folder, run `cargo-build-sbf`. The results will be saved to `target/deploy`.
