use clap::IntoApp;
use clap_generate::generate_to;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use std::{env, fs, path};
use vergen::vergen;

include!("src/cli.rs");

fn main() {
    let mut flags = vergen::Config::default();
    // If passed a version, use that instead of vergen's formatting
    if let Ok(val) = env::var("GWT_VERSION") {
        *flags.git_mut().semver_mut() = false;
        println!("cargo:rustc-env=VERGEN_GIT_SEMVER={}", val)
    };
    // Try to output flags based on Git repo, but if that fails turn off Git features and try again
    // with just cargo generated version info
    if vergen(flags).is_err() {
        *flags.git_mut().semver_mut() = false;
        *flags.git_mut().branch_mut() = false;
        *flags.git_mut().commit_timestamp_mut() = false;
        *flags.git_mut().sha_mut() = false;
        *flags.git_mut().rerun_on_head_change_mut() = false;
        vergen(flags).expect("Unable to generate the cargo keys!");
    }
    generate_shell_completions();
}

/// Generate shell completion files from CLI interface
fn generate_shell_completions() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let completions_dir = path::Path::new(&out_dir).join("completions");
    fs::create_dir_all(&completions_dir)
        .expect("Could not create directory in which to place completions");
    let app = Cli::into_app();
    let bin_name: &str = app
        .get_bin_name()
        .expect("Could not retrieve bin-name from generated Clap app");
    let mut app = Cli::into_app();
    generate_to::<Bash, _, _>(&mut app, bin_name, &completions_dir);
    generate_to::<Elvish, _, _>(&mut app, bin_name, &completions_dir);
    generate_to::<Fish, _, _>(&mut app, bin_name, &completions_dir);
    generate_to::<PowerShell, _, _>(&mut app, bin_name, &completions_dir);
    generate_to::<Zsh, _, _>(&mut app, bin_name, &completions_dir);
}
