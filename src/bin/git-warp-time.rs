use clap::IntoApp;

use git_warp_time::cli::Cli;
use git_warp_time::FileSet;
use git_warp_time::{get_repo, reset_mtime};

fn main() -> git_warp_time::Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let positionals = matches.values_of("paths");
    let repo = get_repo().unwrap();
    let mut opts = git_warp_time::Options::new()
        .dirty(matches.is_present("dirty"))
        .ignored(matches.is_present("ignore"))
        .verbose(!matches.is_present("quiet"));
    if matches.is_present("paths") {
        let mut paths: FileSet = FileSet::new();
        for path in positionals.unwrap() {
            paths.insert(path.to_string());
        }
        opts = opts.paths(Some(paths));
    }
    reset_mtime(repo, opts)?;
    Ok(())
}
