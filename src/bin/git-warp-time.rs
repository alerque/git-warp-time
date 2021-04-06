use clap::IntoApp;
use git_warp_time::cli::Cli;
use git_warp_time::{get_repo, reset_mtime};
use git_warp_time::{Result, VERSION};

fn main() -> Result<()> {
    let app = Cli::into_app().version(VERSION);
    app.get_matches();
    let repo = get_repo()?;
    let files = reset_mtime(repo)?;
    for file in files.iter() {
        println!("Rewound the clock: {}", file);
    }
    Ok(())
}
