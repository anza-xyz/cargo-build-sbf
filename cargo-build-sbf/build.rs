use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

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

    let warning = "Your cargo installation folder `~/.cargo/bin` appears to be after your Solana \
                   installation folder `~/.local/share/solana/install/active_release/bin`. Update \
                   your PATH to place `~/.cargo/bin` before \
                   `~/.local/share/solana/install/active_release/bin`, or this installation won't \
                   be used.";

    // Cargo hides build script warnings for packaged builds (e.g. installs from
    // crates.io), so write those to the terminal directly.
    let is_packaged = env::var_os("CARGO_MANIFEST_DIR")
        .is_some_and(|dir| Path::new(&dir).join("Cargo.toml.orig").exists());
    if is_packaged && let Ok(mut tty) = OpenOptions::new().write(true).open("/dev/tty") {
        // Clear any progress line cargo has drawn, then match cargo's warning
        // style.
        let _ = writeln!(
            tty,
            "\r\x1b[K\x1b[1;33mwarning\x1b[0m: cargo-build-sbf@{}: {warning}",
            env!("CARGO_PKG_VERSION"),
        );
        return;
    }

    std::println!("cargo:warning={warning}");
}
