use std::env;
use vergen::vergen;

fn main() {
    let mut flags = vergen::Config::default();
    // If passed a version, use that instead of vergen's formatting
    if let Ok(val) = env::var("GWT_VERSION") {
        *flags.git_mut().semver_mut() = false;
        println!("cargo:rustc-env=VERGEN_GIT_SEMVER={}", val)
    };
    vergen(flags).expect("Unable to generate the cargo keys!");
}
