here=$(dirname "$0")
source "${here}"/rust-version.sh nightly

rustup component add clippy rustfmt --toolchain "$rust_nightly"
