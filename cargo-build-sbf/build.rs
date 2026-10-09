use std::{env, path::PathBuf};

fn main() {
    let Some(path_variable) = env::var_os("PATH") else {
        return;
    };

    let Some(home_path) = env::var_os("HOME") else {
        return;
    };

    let paths: Vec<PathBuf> = env::split_paths(&path_variable).collect();

    let cargo_bin = PathBuf::from(&home_path).join(".cargo/bin");
    let solana_bin =
        PathBuf::from(&home_path).join(".local/share/solana/install/active_release/bin");

    let Some(cargo_index) = paths.iter().position(|p| p == &cargo_bin) else {
        return;
    };
    let Some(solana_index) = paths.iter().position(|p| p == &solana_bin) else {
        return;
    };

    if solana_index > cargo_index {
        return;
    }

    if solana_bin.join("cargo-build-sbf").is_file() {
        std::println!(
            "cargo:warning=The Solana CLI's `cargo-build-sbf` in \
             `~/.local/share/solana/install/active_release/bin` comes before `~/.cargo/bin` in \
             your PATH, so `cargo build-sbf` will run it instead of this installation. Remove \
             `~/.local/share/solana/install/active_release/bin/cargo-build-sbf`, or update your \
             PATH to place `~/.cargo/bin` before \
             `~/.local/share/solana/install/active_release/bin`."
        );
    } else {
        std::println!(
            "cargo:warning=Your Solana installation folder \
             `~/.local/share/solana/install/active_release/bin` comes before `~/.cargo/bin` in \
             your PATH. If a Solana CLI install or update adds a `cargo-build-sbf` there, `cargo \
             build-sbf` will run it instead of this installation. Update your PATH to place \
             `~/.cargo/bin` before `~/.local/share/solana/install/active_release/bin` to avoid \
             this."
        );
    }
}
