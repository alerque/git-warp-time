use git2::Repository;
use std::collections::HashSet;
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

fn find_candidates(_repo: &Repository) -> Result<FileSet> {
    let candidates = FileSet::new();
    Ok(candidates)
}

fn find_files(_repo: &Repository) -> Result<FileSet> {
    let workdir_files = FileSet::new();
    Ok(workdir_files)
}

fn touch(_repo: &Repository, _touchables: HashSet<&String>) -> Result<FileSet> {
    let touched = FileSet::new();
    Ok(touched)
}
