use {cargo_metadata::camino::Utf8PathBuf, std::path::PathBuf};

#[derive(Debug)]
pub struct Config<'a> {
    pub cargo_args: Vec<&'a str>,
    pub target_directory: Option<Utf8PathBuf>,
    pub sbf_out_dir: Option<PathBuf>,
    pub platform_tools_version: Option<&'a str>,
    pub dump: bool,
    pub features: Vec<String>,
    pub force_tools_install: bool,
    pub skip_tools_install: bool,
    pub no_rustup_override: bool,
    pub generate_child_script_on_failure: bool,
    pub no_default_features: bool,
    pub offline: bool,
    pub remap_cwd: bool,
    pub debug: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub workspace: bool,
    pub jobs: Option<String>,
    pub arch: &'a str,
    pub optimize_size: bool,
    pub lto: bool,
    pub install_only: bool,
    pub patch_binaries_for_nix: Option<bool>,
    pub use_abi_v2: bool,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            cargo_args: vec![],
            target_directory: None,
            sbf_out_dir: None,
            platform_tools_version: None,
            dump: false,
            features: vec![],
            force_tools_install: false,
            skip_tools_install: false,
            no_rustup_override: false,
            generate_child_script_on_failure: false,
            no_default_features: false,
            offline: false,
            remap_cwd: true,
            debug: false,
            verbose: false,
            quiet: false,
            workspace: false,
            jobs: None,
            arch: "v0",
            optimize_size: false,
            lto: false,
            install_only: false,
            patch_binaries_for_nix: None,
            use_abi_v2: false,
        }
    }
}
