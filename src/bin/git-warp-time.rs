use clap::CommandFactory;

use git_warp_time::cli::Cli;
use git_warp_time::FileSet;
use git_warp_time::{get_repo, reset_mtime, resolve_repo_path};

use std::io::{Error, ErrorKind};
use std::path::Path;

fn main() -> git_warp_time::Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let positionals = matches.get_many::<String>("paths");
    let repo = get_repo().unwrap();
    let mut opts = git_warp_time::Options::new()
        .dirty(matches.contains_id("dirty"))
        .ignored(matches.contains_id("ignored"))
        .verbose(!matches.contains_id("quiet"));
    if matches.contains_id("paths") {
        let mut paths: FileSet = FileSet::new();
        for path in positionals.unwrap() {
            if !Path::new(path).exists() {
                let path_error = format!("Path {path} does not exist");
                return Err(Box::new(Error::new(ErrorKind::NotFound, path_error)));
            }
            let path = resolve_repo_path(&repo, path)?;
            paths.insert(path);
        }
        opts = opts.paths(Some(paths));
    }
    reset_mtime(repo, opts)?;
    Ok(())
}
