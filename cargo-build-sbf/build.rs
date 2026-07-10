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

    if solana_index < cargo_index {
        std::println!(
            "cargo:warning=Your cargo installation folder `~/.cargo/bin` appears to be after your \
             Solana installation folder `~/.local/share/solana/install/active_release/bin`. \
             Unless such order is reversed, the Solana provided cargo-build-sbf will hide the new \
             installation."
        );
    }
}
