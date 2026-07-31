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

The deployment of SBPF versions v0, v1 and v2 is going to be blocked once SIMD-500 is activated. It is scheduled for 
activation in Agave v4.3.

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

To check the SBPF version your program was built for, run:

```
$ readelf program.so --header | grep Flags:
  Flags:                             0x0
  Key to Flags:
```

The first line on the output indicates the version. You'll see `Flags: 0x0` for SBPFv0, `Flags: 0x1` for SBPFv1,
`Flags: 0x2` for SBPFv2, and `Flags: 0x3` for SBPFv3.

On MacOS, `binutils` is available at `/opt/homebrew/opt/binutils/bin/readelf` after installing it with 
`brew install binutils`.

Keep an eye on this page, as we may bring SBPFv3 compatibility to more versions of such components to ensure a 
smooth migration.
