#![doc = include_str!("../README.md")]

use filetime::FileTime;
use gix::easy;
use gix::easy::options::DiffOptions;
use gix::odb::pack;
use gix::reference::ReferenceKind;
use gix::repository::{discovery, find, locate};
use gix::Repository;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, error, fs, result};

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub type FileSet = HashSet<String>;

/// Options passed to `reset_mtime()`
#[derive(Clone, Debug)]
pub struct Options {
    paths: Option<FileSet>,
    dirty: bool,
    ignored: bool,
    verbose: bool,
}

/// foo
impl Default for Options {
    /// Return a new options strut with default values.
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    /// Return a set of default options.
    pub fn new() -> Options {
        Options {
            paths: None,
            dirty: false,
            ignored: false,
            verbose: false,
        }
    }

    /// Whether or not to touch locally modified files, default is false
    pub fn dirty(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: flag,
            ignored: self.ignored,
            verbose: self.verbose,
        }
    }

    /// Whether or not to touch ignored files, default is false
    pub fn ignored(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: self.dirty,
            ignored: flag,
            verbose: self.verbose,
        }
    }

    /// Whether or not to print output when touching or skipping files, default is false
    pub fn verbose(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: self.dirty,
            ignored: self.ignored,
            verbose: flag,
        }
    }

    /// List of paths to operate on instead of scanning repository
    pub fn paths(&self, input: Option<FileSet>) -> Options {
        Options {
            paths: input,
            dirty: self.dirty,
            ignored: self.ignored,
            verbose: self.verbose,
        }
    }
}

/// Iterate over either the explicit file list or the working directory files, filter out any that
/// have local modifications, are ignored by Git, or are in submodules and reset the file metadata
/// mtime to the commit date of the last commit that affected the file in question.
pub fn reset_mtime(repo: Repository, opts: Options) -> Result<FileSet> {
    let workdir_files = gather_workdir_files(&repo)?;
    let touchables: FileSet = match opts.paths {
        Some(ref paths) => {
            let not_tracked = paths.difference(&workdir_files);
            if !not_tracked.is_empty() {
                let tracking_error =
                    format!("Paths {:?} are not tracked in the repository", not_tracked);
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    tracking_error,
                )));
            }
            workdir_files.intersection(paths).cloned().collect()
        }
        None => {
            let candidates = gather_index_files(&repo, &opts)?;
            workdir_files.intersection(&candidates).cloned().collect()
        }
    };
    let touched = touch(&repo, touchables, &opts)?;
    Ok(touched)
}

/// Return a repository discovered from from the current working directory or $GIT_DIR settings.
pub fn get_repo() -> Result<Repository> {
    let repo_path = find::repo()?;
    let repo = discovery::Repository::discover(&repo_path)?;
    Ok(repo.into())
}

/// Convert a path relative to the current working directory to be relative to the repository root
pub fn resolve_repo_path(repo: &Repository, path: &String) -> Result<String> {
    let cwd = env::current_dir()?;
    let repo_path = repo.workdir().ok_or("No Git working directory found")?;
    let prefix = cwd.strip_prefix(&repo_path).unwrap();
    let abs_input_path = if Path::new(&path).is_absolute() {
        PathBuf::from(path.clone())
    } else {
        prefix.join(path.clone())
    };
    let resolved_path = abs_input_path.to_string_lossy().to_string();
    Ok(resolved_path)
}

fn gather_index_files(repo: &Repository, opts: &Options) -> FileSet {
    let mut candidates = FileSet::new();
    let odb = repo.odb.into_object_by_path();
    let head_reference = repo.refs.read_reference(ReferenceKind::Head).unwrap();
    let head_commit = gix::easy::peel::peel_commit(&head_reference.target().into())?;
    let head_tree = head_commit.tree(&repo)?;

    let mut options = DiffOptions::new();
    options.include_unmodified(true);
    options.exclude_submodules(true);
    options.include_ignored(opts.ignored);

    let index_tree = gix::easy::index::from_repository(&repo)
        .commit()
        .tree(&repo)?;
    let diff = gix::diff::tree_to_tree(&repo, Some(&options), Some(&index_tree), Some(&head_tree))
        .unwrap();

    for delta in diff.deltas {
        let path = delta.old_file.path().unwrap().to_string();
        match delta.status {
            gix::diff::DeltaStatus::Unmodified => {
                candidates.insert(path);
            }
            gix::diff::DeltaStatus::Modified => {
                if opts.dirty {
                    candidates.insert(path);
                } else if opts.verbose {
                    println!("Ignored file with staged modifications: {}", path);
                }
            }
            gix::diff::DeltaStatus::TypeChange => {
                if opts.verbose {
                    println!("Ignored file in state TypeChange: {}", path);
                }
            }
            _ => {
                if opts.verbose {
                    println!("Ignored file in state {:?}: {}", delta.status, path);
                }
            }
        }
    }
    candidates
}

fn gather_workdir_files(repo: &Repository) -> Result<FileSet> {
    let mut workdir_files = FileSet::new();
    let head_reference = repo.refs.read_reference(ReferenceKind::Head).unwrap();
    let head_commit = gix::easy::peel::peel_commit(&head_reference.target().into())?;
    let head_tree = head_commit.tree(&repo)?;

    head_tree.iter_tree_walk(|dir, entry| {
        let file = format!("{}{}", dir, entry.name().unwrap());
        let path = Path::new(&file);
        if path.is_dir() {
            return Ok(false);
        }
        workdir_files.insert(file);
        Ok(true)
    })?;
    Ok(workdir_files)
}

fn touch(repo: &Repository, touchables: HashSet<String>, opts: &Options) -> Result<FileSet> {
    let mut touched = FileSet::new();
    for path in touchables.iter() {
        let pathbuf = Path::new(path).to_path_buf();
        let head_reference = repo.refs.read_reference(ReferenceKind::Head).unwrap();
        let head_commit = gix::easy::peel::peel_commit(&head_reference.target().into())?;
        let commits = gix::easy::revwalk::commits(head_commit.into());
        let latest_file_oid = commits
            .first()
            .map(|commit| {
                commit
                    .tree(&repo)?
                    .get_path(&pathbuf)?
                    .as_blob()
                    .and_then(|blob| Some(blob.id()))
            })
            .flatten()
            .ok_or(Error::new(
                ErrorKind::Other,
                "Unable to find the latest file OID",
            ))?;
        let mut file_oid = latest_file_oid;
        let mut file_path = pathbuf.to_path_buf();
        for commit in commits.iter() {
            let old_tree = commit.parents.first().and_then(|p| p.tree(&repo));
            let new_tree = commit.tree(&repo)?;
            let diff =
                gix::diff::tree_to_tree(&repo, None, Some(&old_tree), Some(&new_tree)).unwrap();
            let delta = diff.deltas.iter().find(|delta| {
                delta.old_file.path == file_path.to_str().unwrap() && delta.old_file.id == file_oid
            });
            if let Some(delta) = delta {
                file_oid = delta.new_file.id;
                file_path = PathBuf::from(&delta.new_file.path);
            }
        }
        let metadata = fs::metadata(&path).unwrap();
        let commit_time = FileTime::from_unix_time(
            head_commit.time().seconds as i64,
            head_commit.time().nanoseconds as i64,
        );
        let file_mtime = FileTime::from_last_modification_time(&metadata);
        if file_mtime != commit_time {
            filetime::set_file_mtime(&path, commit_time)?;
            if opts.verbose {
                println!("Rewound the clock: {}", path);
            }
            touched.insert(path.to_string());
        }
    }
    Ok(touched)
}
