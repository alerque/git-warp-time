use clap::IntoApp;
use git_warp_time::cli::Cli;
use git_warp_time::{get_repo, reset_mtime};

fn main() -> git_warp_time::Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::into_app().version(version);
    let matches = app.get_matches();
    let repo = get_repo()?;
    let opts = git_warp_time::Options::new().dirty(matches.is_present("dirty"));
    let files = reset_mtime(repo, opts)?;
    for file in files.iter() {
        println!("Rewound the clock: {}", file);
    }
    Ok(())
}
