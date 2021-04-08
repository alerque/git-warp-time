# git warp-time

[![Rust Test Status](https://img.shields.io/github/workflow/status/alerque/git-warp-time/Rust%20Test?label=Rust+Test&logo=Rust)](https://github.com/alerque/git-warp-time/actions?workflow=Rust+Test)
[![Rust Lint Status](https://img.shields.io/github/workflow/status/alerque/git-warp-time/Rust%20Lint?label=Rust+Lint&logo=Rust)](https://github.com/alerque/git-warp-time/actions?workflow=Rust+Lint)

CLI utility (and Rust library) that rewinds the last modified timestamp in filesystem metadata to be the time of the last commit in which each file was modified.

## CLI usage

Run from inside any Git working directory after clone, after any checkout operation that switches branches, after rebases, etc.

```console
$ git clone ‹project›
$ cd ‹project›
$ git warp-time
```

## Library Usage

```toml
[dependencies]
git-warp-time = "0.3.0"
default-features = false
```

```rust
use git_warp_time::{get_repo, reset_mtime};
let repo = get_repo().unwrap();
let paths = git_warp_time::FileSet::new()
	.insert("foo.txt");
let opts = git_warp_time::Options::new()
	.verbose(true).
	.paths(paths);
let files = reset_mtime(repo, opts).unwrap();
println!("Actioned files: {:?}", files);
```

# The story

Whenever you `git clone` a project or `git checkout` a different branch, Git will write all the relevant files to your system at the moment you run the Git command.
Logical enough.
Git is doing the right thing.
For most use cases there is nothing wrong with the latest modification timestamp of a file to be the last time it's state changed on your disk.

However many build systems rely on file modification timestamps to understand when something needs to be rebuilt.
GNU Make is one example that relies entirely on timestamps, but there are many others.
A few rely on checksums (e.g. SCons) and keep a separate database of ‘last seen’ file states, but since this requires extra storage most build systems use what is available.
What is available without the build system storing it's own state is your file system's meta data.

The rub happens when you take advantage of Git's cheap branching model.
Many workflows branch early and branch often.
Every time you `git checkout <branch>`, your local working tree will be updated with the time of your checkout.
In some cases this will cause unnecessary rebuilds.

For many projects these rebuilds are required: if the state of all files at once doesn't match your project won't be built right.
However for some projects, particularly those with multiple outputs, this might be a lot of wasted work.

## A case study

I have one project with many hundreds of LaTeX files.
The projects output is a directory of PDFs in a shared file repository (Nextcloud).
There are currently 5.7 Gigs of PDF files.
Each week this collection grows.
Most of the files are completely independent of each other, but they all use a common template and some other includes.
Most weeks I just add new files and building the project just adds a few more files to the output.
Periodically I will change something in the template that will cause the entire output file set to regenerate.
That process takes about 20 hours to complete.

Git is a distributed version control system and I *should* be able to work on this project from anywhere, but there is a problem.
If I clone the project to a new system, the source files are *all* newer than the existing outputs and the build system can't figure out what actually needs to be rebuilt.
One solution is to `touch` every file in the project after a clone with a very old date.
That sledge hammer approach works well enough for clones, but any time I work in a feature branch things get messed up.
Returning from a feature branch that messes with the template to the master branch will cause the template file to be ‘new’ again and the whole project tries to rebuild.
This utility is a more elegant solution.
Running `git warp-time` after any clone, checkout, rebase, or similar operation will reset all the timestamps to when they were actually last touched by a commit.

The result is portable.
I can clone the project on a new system and without any build state data except the existing output files the project known what it does and doesn't need to rebuild.

## When not to use this

Not all Git projects will benefit from `git warp-time`.

* If your build system doesn't use timestamps, this won't help you.
* If your project generates a single output such as a binary, you probably need to rebuild when *any* of the inputs change so this won't help much.
* If your build system supports incremental builds and you do creative things in your branches, you might completely confuse your build system and cause incomplete builds.
