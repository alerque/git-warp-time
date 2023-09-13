use clap::Parser;

/// CLI utility that resets the timestamps of files in a Git repository working directory
/// to the exact timestamp of the last commit which modified each file.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "git-warp-time")]
pub struct Cli {
    /// Include files tracked by Git but modifications in the working tee
    #[clap(short, long)]
    pub dirty: bool,

    /// Include files tracked by Git but also ignored
    #[clap(short, long)]
    pub ignored: bool,

    /// Don't print any output about files touched or skipped
    #[clap(short, long)]
    pub quiet: bool,

    /// Optional list of paths to operate on instead of default which is all files tracked by Git
    pub paths: Vec<String>,
}
