/// CLI version number as detected by `git describe --tags` at build time
pub static VERSION: &str = env!("VERGEN_GIT_SEMVER");
