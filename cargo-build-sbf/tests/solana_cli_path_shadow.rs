//! Tests for the (temporary) work we're doing to deal with the `PATH`
//! shadowing issue from `cargo-build-sbf` binaries that shipped bundled with
//! the Solana CLI.

use {
    assert_cmd::Command,
    predicates::prelude::*,
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

#[macro_use]
extern crate serial_test;

const WARNING: &str = "was installed with the Solana CLI";

fn fake_home(bin_name: &str) -> PathBuf {
    let home = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("solana_cli_path_shadow")
        .join(bin_name);
    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).unwrap();
    home
}

// Lets the test invoke the executable without having to actually trigger a
// build, ie. `cargo-build-sbf --version`.
fn version_command(exe: &Path, home: &Path) -> Command {
    let mut command = Command::new(exe);
    command
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env_remove("CARGO_HOME")
        .arg("--version");
    command
}

fn assert_warns_from_solana_cli_install(bin_name: &str, exe: &str) {
    let exe = Path::new(exe);
    let home = fake_home(bin_name);

    // Run from the build directory: no warning.
    version_command(exe, &home)
        .assert()
        .success()
        .stderr(predicate::str::contains(WARNING).not());

    // Run from the Solana CLI's install directory: warning.
    let solana_bin = home
        .join(".local")
        .join("share")
        .join("solana")
        .join("install")
        .join("active_release")
        .join("bin");
    fs::create_dir_all(&solana_bin).unwrap();
    let bundled_exe = solana_bin.join(exe.file_name().unwrap());
    fs::copy(exe, &bundled_exe).unwrap();
    version_command(&bundled_exe, &home)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "warning: This `{bin_name}` {WARNING}"
        )));
}

#[test]
#[serial]
fn test_cargo_build_sbf_warns_from_solana_cli_install() {
    assert_warns_from_solana_cli_install("cargo-build-sbf", env!("CARGO_BIN_EXE_cargo-build-sbf"));
}

#[test]
#[serial]
fn test_cargo_test_sbf_warns_from_solana_cli_install() {
    assert_warns_from_solana_cli_install("cargo-test-sbf", env!("CARGO_BIN_EXE_cargo-test-sbf"));
}
