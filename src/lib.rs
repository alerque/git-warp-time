// SPDX-FileCopyrightText: © 2021 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: GPL-3.0-only

#![doc = include_str!("../README.md")]

use snafu::prelude::*;

use camino::{Utf8Path, Utf8PathBuf};
use filetime::FileTime;
use git2::{Diff, Oid, Repository};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::sync::{Arc, RwLock};
use std::{env, fs};

#[cfg(feature = "cli")]
pub mod cli;

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("std::io::Error {}", source))]
    IoError {
        source: std::io::Error,
    },
    #[snafu(display("git2::Error {}", source))]
    LibGitError {
        source: git2::Error,
    },
    #[snafu(display("Paths {} are not tracked in the repository.", paths))]
    PathNotTracked {
        paths: String,
    },
    #[snafu(display("Cannot remove prefix from path:\n{}", source))]
    PathError {
        source: std::path::StripPrefixError,
    },
    #[snafu(display("Path contains invalid Unicode:\n{}", source))]
    PathEncodingError {
        source: camino::FromPathBufError,
    },
    UnresolvedError {},
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type FileSet = HashSet<Utf8PathBuf>;

/// Options passed to `reset_mtimes()`
#[derive(Clone, Debug)]
pub struct Options {
    paths: Option<FileSet>,
    dirty: bool,
    ignored: bool,
    ignore_older: bool,
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
            ignore_older: false,
            verbose: false,
        }
    }

    /// Whether or not to touch locally modified files, default is false
    pub fn dirty(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: flag,
            ignored: self.ignored,
            ignore_older: self.ignore_older,
            verbose: self.verbose,
        }
    }

    /// Whether or not to touch ignored files, default is false
    pub fn ignored(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: self.dirty,
            ignored: flag,
            ignore_older: self.ignore_older,
            verbose: self.verbose,
        }
    }

    /// Whether or not to touch files older than history, default is true
    pub fn ignore_older(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: self.dirty,
            ignored: self.ignored,
            ignore_older: flag,
            verbose: self.verbose,
        }
    }

    /// Whether or not to print output when touching or skipping files, default is false
    pub fn verbose(&self, flag: bool) -> Options {
        Options {
            paths: self.paths.clone(),
            dirty: self.dirty,
            ignored: self.ignored,
            ignore_older: self.ignore_older,
            verbose: flag,
        }
    }

    /// List of paths to operate on instead of scanning repository
    pub fn paths(&self, input: Option<FileSet>) -> Options {
        Options {
            paths: input,
            dirty: self.dirty,
            ignored: self.ignored,
            ignore_older: self.ignore_older,
            verbose: self.verbose,
        }
    }
}

/// Iterate over either the explicit file list or the working directory files, filter out any that
/// have local modifications, are ignored by Git, or are in submodules and reset the file metadata
/// mtime to the commit date of the last commit that affected the file in question.
pub fn reset_mtimes(repo: Repository, opts: Options) -> Result<FileSet> {
    let workdir_files = gather_workdir_files(&repo)?;
    let touchables: FileSet = match opts.paths {
        Some(ref paths) => {
            let not_tracked = paths.difference(&workdir_files);
            if not_tracked.clone().count() > 0 {
                let not_tracked = format!("{not_tracked:?}");
                return PathNotTrackedSnafu { paths: not_tracked }.fail();
            }
            workdir_files.intersection(paths).cloned().collect()
        }
        None => {
            let candidates = gather_index_files(&repo, &opts)?;
            workdir_files.intersection(&candidates).cloned().collect()
        }
    };
    let touched = process_touchables(&repo, touchables, &opts)?;
    Ok(touched)
}

/// Return a repository discovered from from the current working directory or $GIT_DIR settings.
pub fn get_repo() -> Result<Repository> {
    let repo = Repository::open_from_env().context(LibGitSnafu)?;
    Ok(repo)
}

/// Convert a path relative to the current working directory to be relative to the repository root
pub fn resolve_repo_path(repo: &Repository, path: impl Into<Utf8PathBuf>) -> Result<Utf8PathBuf> {
    let path: Utf8PathBuf = path.into();
    let cwd: Utf8PathBuf = env::current_dir()
        .context(IoSnafu)?
        .try_into()
        .context(PathEncodingSnafu)?;
    let root = repo.workdir().context(UnresolvedSnafu)?;
    let prefix: Utf8PathBuf = cwd.strip_prefix(root).context(PathSnafu)?.into();
    let resolved_path = if path.is_absolute() {
        path
    } else {
        prefix.join(path)
    };
    Ok(resolved_path)
}

fn gather_index_files(repo: &Repository, opts: &Options) -> Result<FileSet> {
    let mut candidates = FileSet::new();
    let mut status_options = git2::StatusOptions::new();
    status_options
        .include_unmodified(true)
        .exclude_submodules(true)
        .include_ignored(opts.ignored)
        .show(git2::StatusShow::IndexAndWorkdir);
    let statuses = repo
        .statuses(Some(&mut status_options))
        .context(LibGitSnafu)?;
    for entry in statuses.iter() {
        let path = entry.path().context(UnresolvedSnafu)?;
        match entry.status() {
            git2::Status::CURRENT => {
                candidates.insert(path.into());
            }
            git2::Status::INDEX_MODIFIED => {
                if opts.dirty {
                    candidates.insert(path.into());
                } else if opts.verbose {
                    println!("Ignored file with staged modifications: {path}");
                }
            }
            git2::Status::WT_MODIFIED => {
                if opts.dirty {
                    candidates.insert(path.into());
                } else if opts.verbose {
                    println!("Ignored file with local modifications: {path}");
                }
            }
            git_state => {
                if opts.verbose {
                    println!("Ignored file in state {git_state:?}: {path}");
                }
            }
        }
    }
    Ok(candidates)
}

fn gather_workdir_files(repo: &Repository) -> Result<FileSet> {
    let mut workdir_files = FileSet::new();
    let head = repo.head().context(LibGitSnafu)?;
    let tree = head.peel_to_tree().context(LibGitSnafu)?;
    tree.walk(git2::TreeWalkMode::PostOrder, |dir, entry| {
        if let Some(name) = entry.name() {
            let file = format!("{}{}", dir, name);
            let path = Utf8Path::new(&file);
            if path.is_dir() {
                return git2::TreeWalkResult::Skip;
            }
            workdir_files.insert(file.into());
        }
        git2::TreeWalkResult::Ok
    })
    .context(LibGitSnafu)?;
    Ok(workdir_files)
}

fn diff_affects_oid(diff: &Diff, oid: &Oid, touchable_path: &mut Utf8PathBuf) -> bool {
    diff.deltas().any(|delta| {
        delta.new_file().id() == *oid
            && delta
                .new_file()
                .path()
                .filter(|path| *path == touchable_path)
                .is_some()
    })
}

fn touch_if_time_mismatch(
    path: Utf8PathBuf,
    time: i64,
    verbose: bool,
    ignore_older: bool,
) -> Result<bool> {
    let commit_time = FileTime::from_unix_time(time, 0);
    let metadata = fs::metadata(&path).context(IoSnafu)?;
    let file_mtime = FileTime::from_last_modification_time(&metadata);
    if file_mtime > commit_time || (!ignore_older && file_mtime < commit_time) {
        filetime::set_file_mtime(&path, commit_time).context(IoSnafu)?;
        if verbose {
            println!("Rewound the clock: {path}");
        }
        return Ok(true);
    }
    Ok(false)
}

fn process_touchables(repo: &Repository, touchables: FileSet, opts: &Options) -> Result<FileSet> {
    let touched = Arc::new(RwLock::new(FileSet::new()));
    let mut touchable_oids: HashMap<Oid, Utf8PathBuf> = HashMap::new();
    let mut revwalk = repo.revwalk().context(LibGitSnafu)?;
    // See https://github.com/arkark/git-hist/blob/main/src/app/git.rs
    revwalk.push_head().context(LibGitSnafu)?;
    revwalk.simplify_first_parent().context(LibGitSnafu)?;
    let commits: Vec<_> = revwalk
        .map(|oid| oid.and_then(|oid| repo.find_commit(oid)).unwrap())
        .collect();
    let latest_tree = commits
        .first()
        .context(UnresolvedSnafu)?
        .tree()
        .context(LibGitSnafu)?;
    touchables.iter().for_each(|path| {
        let touchable_path: Utf8PathBuf = path.into();
        let current_oid = latest_tree
            .get_path(&touchable_path.clone().into_std_path_buf())
            .and_then(|entry| {
                if let Some(git2::ObjectType::Blob) = entry.kind() {
                    Ok(entry)
                } else {
                    Err(git2::Error::new(
                        git2::ErrorCode::NotFound,
                        git2::ErrorClass::Tree,
                        "no blob",
                    ))
                }
            })
            .unwrap()
            .id();
        touchable_oids.insert(current_oid, touchable_path);
    });
    commits.iter().try_for_each(|commit| {
        let old_tree = commit.parent(0).and_then(|p| p.tree()).ok();
        let new_tree = commit.tree().ok();
        let mut diff = repo
            .diff_tree_to_tree(old_tree.as_ref(), new_tree.as_ref(), None)
            .unwrap();
        diff.find_similar(Some(git2::DiffFindOptions::new().renames(true)))
            .unwrap();
        touchable_oids.retain(|oid, touchable_path| {
            let affected = diff_affects_oid(&diff, oid, touchable_path);
            if affected {
                let time = commit.time().seconds();
                if let Ok(true) = touch_if_time_mismatch(
                    touchable_path.to_path_buf(),
                    time,
                    opts.verbose,
                    opts.ignore_older,
                ) {
                    touched
                        .write()
                        .unwrap()
                        .insert(touchable_path.to_path_buf());
                }
            }
            !affected
        });
        if !touchable_oids.is_empty() {
            Some(())
        } else {
            None
        }
    });
    let touched: RwLock<FileSet> = Arc::into_inner(touched).unwrap();
    let touched: FileSet = RwLock::into_inner(touched).unwrap();
    Ok(touched)
}
