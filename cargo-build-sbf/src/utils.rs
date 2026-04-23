use {
    itertools::Itertools,
    log::{error, info},
    regex::Regex,
    solana_keypair::{Keypair, write_keypair_file},
    std::{
        env,
        ffi::OsStr,
        fs::File,
        io::{BufWriter, Write},
        path::{Path, PathBuf},
        process::{Command, Stdio, exit},
    },
};

pub fn spawn<I, S>(program: &Path, args: I, generate_child_script_on_failure: bool) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args = Vec::from_iter(args);
    let msg = args
        .iter()
        .map(|arg| arg.as_ref().to_str().unwrap_or("?"))
        .join(" ");
    info!("spawn: {program:?} {msg}");

    let child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| {
            error!("Failed to execute {}: {}", program.display(), err);
            exit(1);
        });

    let output = child.wait_with_output().expect("failed to wait on child");
    if !output.status.success() {
        if !generate_child_script_on_failure {
            exit(1);
        }
        error!("cargo-build-sbf exited on command execution failure");
        let script_name = format!(
            "cargo-build-sbf-child-script-{}.sh",
            program.file_name().unwrap().to_str().unwrap(),
        );
        let file = File::create(&script_name).unwrap();
        let mut out = BufWriter::new(file);
        for (key, value) in env::vars() {
            writeln!(out, "{key}=\"{value}\" \\").unwrap();
        }
        write!(out, "{}", program.display()).unwrap();
        writeln!(out, "{msg}").unwrap();
        out.flush().unwrap();
        error!("To rerun the failed command for debugging use {script_name}");
        exit(1);
    }
    output
        .stdout
        .as_slice()
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
}

pub(crate) fn create_directory(path: &PathBuf) {
    std::fs::create_dir_all(path).unwrap_or_else(|err| {
        error!("Failed create folder: {err}");
        exit(1);
    });
}

pub(crate) fn copy_file(from: &Path, to: &Path) {
    std::fs::copy(from, to).unwrap_or_else(|err| {
        error!("Failed to copy file: {err}");
        exit(1);
    });
}

pub(crate) fn generate_keypair(path: &PathBuf) {
    write_keypair_file(&Keypair::new(), path).unwrap_or_else(|err| {
        error!("Unable to create {}: {err}", path.display());
        exit(1);
    });
}

pub fn home_dir() -> PathBuf {
    PathBuf::from(
        #[cfg_attr(not(windows), allow(clippy::unnecessary_lazy_evaluations))]
        env::var_os("HOME")
            .or_else(|| {
                #[cfg(windows)]
                {
                    log::debug!(
                        "Could not read env variable 'HOME', falling back to 'USERPROFILE'"
                    );
                    env::var_os("USERPROFILE")
                }

                #[cfg(not(windows))]
                {
                    None
                }
            })
            .unwrap_or_else(|| {
                error!("Can't get home directory path");
                exit(1);
            }),
    )
}

pub fn is_version_string(arg: &str) -> Result<(), String> {
    let semver_re = Regex::new(r"^v?[0-9]+\.[0-9]+(\.[0-9]+)?$").unwrap();
    if semver_re.is_match(arg) {
        return Ok(());
    }
    Err(
        "a version string may start with 'v' and contains major and minor version numbers \
         separated by a dot, e.g. v1.32 or 1.32"
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_version_string_valid_versions() {
        // Test valid versions that should pass validation
        assert!(is_version_string("1.2.3").is_ok());
        assert!(is_version_string("v2.1.0").is_ok());
        assert!(is_version_string("1.32").is_ok());
        assert!(is_version_string("v1.32").is_ok());
        assert!(is_version_string("0.1").is_ok());
        assert!(is_version_string("v0.1").is_ok());
        assert!(is_version_string("10.20.30").is_ok());
        assert!(is_version_string("v10.20.30").is_ok());
    }

    #[test]
    fn test_is_version_string_invalid_versions() {
        // Test invalid versions that should fail validation
        assert!(is_version_string("1.2.3abc").is_err());
        assert!(is_version_string("v2.1.0-extra").is_err());
        assert!(is_version_string("abc1.2.3").is_err());
        assert!(is_version_string("1").is_err());
        assert!(is_version_string("v1").is_err());
        assert!(is_version_string("1.2.3.4.5").is_err());
        assert!(is_version_string("").is_err());
        assert!(is_version_string("v").is_err());
        assert!(is_version_string("1.").is_err());
        assert!(is_version_string("v1.").is_err());
        assert!(is_version_string(".1.2").is_err());
        assert!(is_version_string("1.2.3-beta").is_err());
        assert!(is_version_string("v1.2.3+build").is_err());
    }

    #[test]
    fn test_is_version_string_error_message() {
        // Test that error message is descriptive
        let result = is_version_string("invalid");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("version string may start with 'v'"));
        assert!(error_msg.contains("major and minor version numbers"));
        assert!(error_msg.contains("separated by a dot"));
    }
}
