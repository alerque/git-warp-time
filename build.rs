use std::env;
use vergen::vergen;

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
}
