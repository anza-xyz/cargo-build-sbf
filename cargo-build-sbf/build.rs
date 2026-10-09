use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc,
    thread,
    time::Duration,
};

const SHADOWED_BINARIES: [&str; 2] = ["cargo-build-sbf", "cargo-test-sbf"];
const PROMPT_TIMEOUT: Duration = Duration::from_secs(60);

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
    if is_packaged && let Ok(mut tty) = OpenOptions::new().read(true).write(true).open("/dev/tty") {
        // Each message first clears any progress line cargo has drawn.
        let _ = if solana_bin.join("cargo-build-sbf").is_file() {
            wait_for_other_jobs();
            prompt(&mut tty, &solana_bin)
        } else {
            writeln!(
                tty,
                "\r\x1b[K\x1b[1;33mwarning\x1b[0m: cargo-build-sbf@{}: {warning}",
                env!("CARGO_PKG_VERSION"),
            )
        };
        return;
    }

    std::println!("cargo:warning={warning}");
}

// Offers to disable the Solana CLI's copies by renaming them, which can be
// undone if this installation later fails. Defaults to leaving them in place.
fn prompt(tty: &mut File, solana_bin: &Path) -> std::io::Result<()> {
    let version = binary_version(&solana_bin.join("cargo-build-sbf"))
        .map(|version| format!(" (v{version})"))
        .unwrap_or_default();
    write!(
        tty,
        "\r\x1b[K\n\x1b[1;33mwarning\x1b[0m: `cargo build-sbf` will not use the version you are \
         installing.\n\nThe Solana CLI ships its own copy{version} at:\n  \
         ~/.local/share/solana/install/active_release/bin/cargo-build-sbf\n\nThat directory comes \
         before ~/.cargo/bin in your PATH, so `cargo build-sbf`\nwill keep running the Solana \
         CLI's copy.\n\nDisable the Solana CLI's copy? [y/N] (defaults to N in {}s) ",
        PROMPT_TIMEOUT.as_secs(),
    )?;
    tty.flush()?;

    let reader = tty.try_clone()?;
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut answer = String::new();
        if BufReader::new(reader).read_line(&mut answer).is_ok() {
            let _ = sender.send(answer);
        }
    });
    let answer = match receiver.recv_timeout(PROMPT_TIMEOUT) {
        Ok(answer) => answer,
        // The user never pressed Enter, so end the prompt line ourselves.
        Err(_) => {
            writeln!(tty)?;
            String::new()
        }
    };
    let answer = answer.trim();
    if !answer.eq_ignore_ascii_case("y") && !answer.eq_ignore_ascii_case("yes") {
        return writeln!(
            tty,
            "Left ~/.local/share/solana/install/active_release/bin/cargo-build-sbf in \
             place.\n`cargo build-sbf` will keep using it until you remove it or move \
             ~/.cargo/bin\nbefore ~/.local/share/solana/install/active_release/bin in your PATH.\n"
        );
    }

    for name in SHADOWED_BINARIES {
        let binary = solana_bin.join(name);
        if !binary.is_file() {
            continue;
        }
        let disabled_name = format!("{name}.disabled");
        match fs::rename(&binary, binary.with_file_name(&disabled_name)) {
            Ok(()) => writeln!(
                tty,
                "Renamed ~/.local/share/solana/install/active_release/bin/{name} to \
                 {disabled_name}."
            )?,
            Err(err) => writeln!(
                tty,
                "Could not rename ~/.local/share/solana/install/active_release/bin/{name}: {err}"
            )?,
        }
    }
    writeln!(
        tty,
        "`cargo build-sbf` will use this installation once it finishes. Updating the Solana CLI \
         will\nrestore the bundled copy.\n"
    )
}

// Waits until this build script is cargo's only running job, so cargo's output
// doesn't interleave with the prompt. The rest of this crate's build depends on
// this build script, so nothing new can start once the other jobs finish.
// Idleness must hold across two polls, in case cargo is between jobs.
#[cfg(unix)]
fn wait_for_other_jobs() {
    const POLL_INTERVAL: Duration = Duration::from_millis(250);
    const WAIT_LIMIT: Duration = Duration::from_secs(300);

    let cargo_pid = std::os::unix::process::parent_id().to_string();
    let start = std::time::Instant::now();
    let mut was_idle = false;
    while start.elapsed() < WAIT_LIMIT {
        let Ok(output) = Command::new("ps").args(["-A", "-o", "ppid="]).output() else {
            return;
        };
        let is_idle = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|ppid| ppid.trim() == cargo_pid)
            .count()
            <= 1;
        if is_idle && was_idle {
            return;
        }
        was_idle = is_idle;
        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(not(unix))]
fn wait_for_other_jobs() {}

// Reads the version from `<binary> --version`, whose first line is
// `cargo-build-sbf <version>`.
fn binary_version(binary: &Path) -> Option<String> {
    let output = Command::new(binary).arg("--version").output().ok()?;
    let stdout = String::from_utf8(output.stdout).ok()?;
    stdout
        .lines()
        .next()?
        .split_whitespace()
        .nth(1)
        .map(str::to_owned)
}
