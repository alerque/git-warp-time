use clap::Parser;

/// CLI utility that operates on the current working tree, resetting file modification timestamps
/// to the date of the last commit in which they were modified.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "git-warp-time")]
pub struct Cli {
    /// Include locally modified files
    #[clap(short, long)]
    pub dirty: bool,

    /// Include ignored files
    #[clap(short, long)]
    pub ignored: bool,

    /// Don't print anything about files touched or skipped
    #[clap(short, long)]
    pub quiet: bool,

    /// Optional list of paths to operate on instead of scannning (relative to repository root)
    pub paths: Vec<String>,
}
