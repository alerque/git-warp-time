use clap::IntoApp;
use git_warp_time::cli::Cli;
use git_warp_time::Result;
use git_warp_time::{get_repo, reset_mtime};

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::into_app().version(version);
    app.get_matches();
    let repo = get_repo()?;
    let files = reset_mtime(repo)?;
    for file in files.iter() {
        println!("Rewound the clock: {}", file);
    }
    Ok(())
}
