use filetime::FileTime;
use git2::Repository;
use std::collections::HashSet;
use std::path::Path;
use std::{error, fs, result};

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

type FileSet = HashSet<String>;

/// Iterate over the working directory files, filter out any that have local modifications, are
/// ignored by Git, or are in submodules and reset the file metadata mtime to the commit date of
/// the last commit that affected the file in question.
pub fn reset_mtime(repo: Repository) -> Result<FileSet> {
    let candidates = find_candidates(&repo);
    let workdir_files = find_files(&repo)?;
    let f: HashSet<_> = workdir_files.intersection(&candidates).collect();
    let touched = touch(&repo, f)?;
    Ok(touched)
}

/// Return a repository discovered from from the current working directory or $GIT_DIR settings.
pub fn get_repo() -> Result<Repository> {
    Ok(Repository::open_from_env()?)
}

fn find_candidates(repo: &Repository) -> FileSet {
    let mut candidates = FileSet::new();
    let mut opts = git2::StatusOptions::new();
    opts.include_unmodified(true)
        .exclude_submodules(true)
        .show(git2::StatusShow::IndexAndWorkdir);
    let statuses = repo.statuses(Some(&mut opts)).unwrap();
    for entry in statuses.iter() {
        let path = entry.path().unwrap();
        match entry.status() {
            git2::Status::CURRENT => {
                candidates.insert(path.to_string());
            }
            git2::Status::WT_MODIFIED => {
                println!("Ignored file with local modifications: {}", path);
            }
            git_state => {
                println!("Ignored file in state {:?}: {}", git_state, path);
            }
        }
    }
    candidates
}

fn find_files(repo: &Repository) -> Result<FileSet> {
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

fn touch(repo: &Repository, touchables: HashSet<&String>) -> Result<FileSet> {
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
            touched.insert((*path).to_string());
        }
    }
    Ok(touched)
}
