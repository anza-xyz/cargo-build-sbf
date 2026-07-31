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

## SBFPv3 migration

Solana is deprecating SBPF versions v0, v1, and v2 in favor of SBPFv3 that enhances cluster security and improves
network performance.

The deployment of SBPF versions v0, v1 and v2 is going to be blocked starting on Agave v4.2, so we are laying out a
plan for migration.

The table below shows the minimum versions of each program component:

| Component                         | Minimum compatible version |
|-----------------------------------|----------------------------|
| Platform tools                    | v1.53                      |
| cargo-build-sbf                   | v4.0.0                     |
| Solana CLI                        | v4.0.0                     |
| Solana SDK (define-syscall crate) | v2.3.0                     |
| Pinocchio                         | v0.10                      |

To build a program for SBPFv3, make sure you have the versions pointed above or newer, and run 
`cargo-build-sbf --arch v3`.

Keep an eye on this page, as we may bring SBPFv3 compatibility to more versions of such components to ensure a 
smooth migration.
