use clap::Clap;

/// Reset file modification timestamps to the time they were last modified in Git version history.
#[derive(Clap, Debug)]
#[clap(bin_name = "git-time-warp")]
pub struct Cli {}
