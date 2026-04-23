# cargo-build-sbf

This repository holds the code for both `cargo-build-sbf` and `cargo-test-sbf` tools used to
build Rust programs for Solana.

Despite the name, the files for the C SDK are also contained here.

## Installation

We bundled `cargo-build-sbf` and `cargo-test-sbf` in the same package, so their installation
is only a single command:

```
cargo install cargo-build-sbf
```

The C SDK consists of a tarball `sbf-sdk.tar.bz2` containing the necessary files to build a 
C program for Solana. Its releases are independently posted on the GitHub releases page.
