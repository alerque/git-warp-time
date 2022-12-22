use clap::CommandFactory;

use git_warp_time::cli::Cli;
use git_warp_time::FileSet;
use git_warp_time::{get_repo, reset_mtime};

fn main() -> git_warp_time::Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let positionals = matches.get_many::<String>("paths");
    let repo = get_repo().unwrap();
    let mut opts = git_warp_time::Options::new()
        .dirty(matches.contains_id("dirty"))
        .ignored(matches.contains_id("ignore"))
        .verbose(!matches.contains_id("quiet"));
    if matches.contains_id("paths") {
        let mut paths: FileSet = FileSet::new();
        for path in positionals.unwrap() {
            paths.insert(path.to_string());
        }
        opts = opts.paths(Some(paths));
    }
    reset_mtime(repo, opts)?;
    Ok(())
}
