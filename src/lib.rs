use filetime::FileTime;
use git2::Repository;
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
            if not_tracked.clone().count() > 0 {
                let tracking_error =
                    format!("Paths {not_tracked:?} are not tracked in the repository");
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    tracking_error,
                )));
            }
            workdir_files.intersection(paths).cloned().collect()
        }
        None => {
            let candidates = gather_index_files(&repo, &opts);
            workdir_files.intersection(&candidates).cloned().collect()
        }
    };
    let touched = touch(&repo, touchables, &opts)?;
    Ok(touched)
}

/// Return a repository discovered from from the current working directory or $GIT_DIR settings.
pub fn get_repo() -> Result<Repository> {
    Ok(Repository::open_from_env()?)
}

/// Convert a path relative to the current working directory to be relative to the repository root
pub fn resolve_repo_path(repo: &Repository, path: &String) -> Result<String> {
    let cwd = env::current_dir()?;
    let root = repo
        .workdir()
        .ok_or("No Git working directory found")?
        .to_path_buf();
    let prefix = cwd.strip_prefix(&root).unwrap();
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
    let mut status_options = git2::StatusOptions::new();
    status_options
        .include_unmodified(true)
        .exclude_submodules(true)
        .include_ignored(opts.ignored)
        .show(git2::StatusShow::IndexAndWorkdir);
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();
    for entry in statuses.iter() {
        let path = entry.path().unwrap();
        match entry.status() {
            git2::Status::CURRENT => {
                candidates.insert(path.to_string());
            }
            git2::Status::INDEX_MODIFIED => {
                if opts.dirty {
                    candidates.insert(path.to_string());
                } else if opts.verbose {
                    println!("Ignored file with staged modifications: {path}");
                }
            }
            git2::Status::WT_MODIFIED => {
                if opts.dirty {
                    candidates.insert(path.to_string());
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
    candidates
}

fn gather_workdir_files(repo: &Repository) -> Result<FileSet> {
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

fn touch(repo: &Repository, touchables: HashSet<String>, opts: &Options) -> Result<FileSet> {
    let mut touched = FileSet::new();
    for path in touchables.iter() {
        let pathbuf = Path::new(path).to_path_buf();
        let mut revwalk = repo.revwalk().unwrap();
        // See https://github.com/arkark/git-hist/blob/main/src/app/git.rs
        revwalk.push_head().unwrap();
        revwalk.simplify_first_parent().unwrap();
        let commits: Vec<_> = revwalk
            .map(|oid| oid.and_then(|oid| repo.find_commit(oid)).unwrap())
            .collect();
        let latest_file_oid = commits
            .first()
            .unwrap()
            .tree()
            .unwrap()
            .get_path(&pathbuf)
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
        let mut file_oid = latest_file_oid;
        let mut file_path = pathbuf;
        let last_commit = commits
            .iter()
            .filter_map(|commit| {
                let old_tree = commit.parent(0).and_then(|p| p.tree()).ok();
                let new_tree = commit.tree().ok();
                let mut diff = repo
                    .diff_tree_to_tree(old_tree.as_ref(), new_tree.as_ref(), None)
                    .unwrap();
                diff.find_similar(Some(git2::DiffFindOptions::new().renames(true)))
                    .unwrap();
                let delta = diff.deltas().find(|delta| {
                    delta.new_file().id() == file_oid
                        && delta
                            .new_file()
                            .path()
                            .filter(|path| *path == file_path)
                            .is_some()
                });
                if let Some(delta) = delta.as_ref() {
                    file_oid = delta.old_file().id();
                    file_path = delta.old_file().path().unwrap().to_path_buf();
                }
                delta.map(|_| commit)
            })
            .next()
            .unwrap();
        let metadata = fs::metadata(path).unwrap();
        let commit_time = FileTime::from_unix_time(last_commit.time().seconds(), 0);
        let file_mtime = FileTime::from_last_modification_time(&metadata);
        if file_mtime != commit_time {
            filetime::set_file_mtime(path, commit_time)?;
            if opts.verbose {
                println!("Rewound the clock: {path}");
            }
            touched.insert((*path).to_string());
        }
    }
    Ok(touched)
}
