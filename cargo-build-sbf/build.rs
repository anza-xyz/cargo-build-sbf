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
        let shadow = solana_bin.join("cargo-build-sbf");
        if shadow.is_file() {
            panic!(
                "\nA version of `cargo-build-sbf` bundled with the Solana CLI at {shadow} will \
                 shadow this installation.\n\nUpdate your PATH to place {cargo_bin} before \
                 {solana_bin}.\n\nAlternatively, delete or rename {shadow}, though updating the \
                 Solana CLI will restore it.\n",
                shadow = shadow.display(),
                cargo_bin = cargo_bin.display(),
                solana_bin = solana_bin.display(),
            );
        }
    }
}
