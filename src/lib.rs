use git2::Repository;
use std::collections::HashSet;
use std::path::Path;
use std::{error, result};

pub mod cli;

/// CLI version number as detected by `git describe --tags` at build time
pub static VERSION: &str = env!("VERGEN_GIT_SEMVER");

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

type FileSet = HashSet<String>;

pub fn reset_mtime(repo: Repository) -> Result<FileSet> {
    let candidates = find_candidates(&repo)?;
    let workdir_files = find_files(&repo)?;
    let f: HashSet<_> = workdir_files.intersection(&candidates).collect();
    let touched = touch(&repo, f)?;
    Ok(touched)
}

/// Get repository object from current working directory
pub fn get_repo() -> Result<Repository> {
    Ok(Repository::discover("./")?)
}

pub fn find_candidates(repo: &Repository) -> Result<FileSet> {
    let mut candidates = FileSet::new();
    let mut opts = git2::StatusOptions::new();
    opts.include_unmodified(true)
        .exclude_submodules(true)
        .show(git2::StatusShow::IndexAndWorkdir);
    let statuses = repo.statuses(Some(&mut opts)).unwrap();
    for entry in statuses.iter() {
        match entry.status() {
            git2::Status::CURRENT => {
                let path = entry.path().unwrap();
                candidates.insert(path.to_string());
            }
            _ => {}
        }
    }
    Ok(candidates)
}

pub fn find_files(repo: &Repository) -> Result<FileSet> {
    let mut workdir_files = FileSet::new();
    let head = repo.head()?;
    let tree = head.peel_to_tree()?;
    tree.walk(git2::TreeWalkMode::PostOrder, |dir, entry| {
        let file = format!("{}{}", dir, entry.name().unwrap());
        let path = Path::new(&file);
        if path.is_dir() {
            return git2::TreeWalkResult::Skip;
        }
        workdir_files.insert(file);
        git2::TreeWalkResult::Ok
    })
    .unwrap();
    Ok(workdir_files)
}

fn touch(_repo: &Repository, _touchables: HashSet<&String>) -> Result<FileSet> {
    let touched = FileSet::new();
    Ok(touched)
}
