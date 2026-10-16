//! Utils for the (temporary) work we're doing to deal with the `PATH`
//! shadowing issue from `cargo-build-sbf` binaries that shipped bundled with
//! the Solana CLI.

use std::{
    env,
    path::{Path, PathBuf},
};

fn solana_cli_bin_dir(home: &Path) -> PathBuf {
    home.join(".local")
        .join("share")
        .join("solana")
        .join("install")
        .join("active_release")
        .join("bin")
}

fn is_same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Warns when the running binary is the copy bundled with the Solana CLI, which
/// shadows a binary installed via `cargo install` whenever the Solana CLI comes
/// first in `PATH`.
///
/// Works for both `build-sbf` and `test-sbf`.
pub fn warn_if_installed_with_solana_cli() {
    let (Ok(exe), Some(home)) = (env::current_exe(), env::home_dir()) else {
        return;
    };
    let Some(file_name) = exe.file_name() else {
        return;
    };
    let solana_bin = solana_cli_bin_dir(&home);
    let bundled_exe = solana_bin.join(file_name);
    if !is_same_file(&exe, &bundled_exe) {
        return;
    }
    let bin_name = exe.file_stem().unwrap_or_default().to_string_lossy();
    let cargo_bin = env::var_os("CARGO_HOME")
        .map_or_else(|| home.join(".cargo"), PathBuf::from)
        .join("bin");
    eprintln!(
        "warning: This `{bin_name}` was installed with the Solana CLI. Install the latest version \
         with `cargo install cargo-build-sbf`, then either delete or rename `{}`, or place `{}` \
         before `{}` in your PATH.",
        bundled_exe.display(),
        cargo_bin.display(),
        solana_bin.display(),
    );
}
