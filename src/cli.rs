use clap::Clap;

/// CLI utility that operates on the current working tree, resetting file modification timestamps
/// to the date of the last commit in which they were modified.
#[derive(Clap, Debug)]
#[clap(bin_name = "git-warp-time")]
pub struct Cli {}
