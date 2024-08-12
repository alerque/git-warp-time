// SPDX-FileCopyrightText: © 2021 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;

/// CLI utility that resets the timestamps of files in a Git repository working directory
/// to the exact timestamp of the last commit which modified each file.
#[derive(Parser, Debug)]
#[command(author, bin_name = "git-warp-time")]
pub struct Cli {
    /// Include files tracked by Git but modifications in the working tee
    #[arg(short, long)]
    pub dirty: bool,

    /// Include files tracked by Git but also ignored
    #[arg(short, long)]
    pub ignored: bool,

    /// Only touch files that are newer than their history, ignore ones that are older
    #[arg(short = 'o', long)]
    pub ignore_older: bool,

    /// Don't print any output about files touched or skipped
    #[arg(short, long)]
    pub quiet: bool,

    /// Optional list of paths to operate on instead of default which is all files tracked by Git
    #[arg(value_hint = clap::ValueHint::FilePath)]
    pub paths: Option<Vec<String>>,
}
