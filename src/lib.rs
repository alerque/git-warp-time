use std::{error, result};

pub mod cli;

/// CLI version number as detected by `git describe --tags` at build time
pub static VERSION: &str = env!("VERGEN_GIT_SEMVER");

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
