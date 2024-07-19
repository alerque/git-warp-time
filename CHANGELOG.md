## [0.8.0] - 2024-07-19

### Features

- *(cli)* Return readable messages on runtime errors
- *(lib)* Return all errors from the library as snafu types
- *(lib)* Simplify path handling by limiting to Unicode

### Refactor

- [**breaking**] Return errors via Snafu rather than panicking

## [0.7.5] - 2024-04-08

### Bug Fixes

- *(build)* Correct typo in cleanup command

## [0.7.2] - 2023-12-11

### Bug Fixes

- *(build)* Only require docker for developer mode builds
- *(actions)* Fix GH Action default arguments

## [0.7.1] - 2023-12-11

### Features

- Add GitHub Action configuration
- Add Dockerfile

## [0.7.0] - 2023-12-08

### Refactor

- [**breaking**] Change both input and output from strings to PathBufs

## [0.6.1] - 2023-12-07

### Bug Fixes

- Restore return of touched FileSet for use as lib

## [0.6.0] - 2023-12-07

### Features

- Process files in parallel

## [0.5.3] - 2023-12-07

### Bug Fixes

- *(cli)* Wire up CLI flags so to take effect again

## [0.5.2] - 2023-11-03

### Bug Fixes

- *(build)* Move build-time dependency checks out of runtime dep check configure flag
- *(build)* Make sure build target doesn't exit with success if it actually fails
- *(build)* Correct package name snafu in boilerplate

## [0.5.0] - 2023-09-13

### Features

- Throw error if input path does not exist
- Resolve relative input paths to repository paths

### Bug Fixes

- Correct use of Clap API, match full not partial option ID

## [0.4.11] - 2023-07-04

### Features

- Add Nix Flake to repository

## [0.4.9] - 2023-01-11

### Bug Fixes

- *(build)* Allow building as library without shell completions

## [0.4.8] - 2022-12-23

### Bug Fixes

- *(build)* Update vergen usage to work on source tarballs

## [0.4.6] - 2022-12-22

### Features

- *(build)* Generate and install manpage

## [0.4.5] - 2022-03-03

### Bug Fixes

- *(build)* Correct local installation check

## [0.4.4] - 2022-01-06

### Bug Fixes

- Swap unportable ‘cp -bf’ for ‘install’ (#1)

## [0.4.3] - 2021-04-16

### Features

- Add feature flags for shell completion generators

### Bug Fixes

- ‌ Correct autoconf metadata

## [0.4.2] - 2021-04-14

### Bug Fixes

- Use Cargo's out_dir so build works on crates.io

## [0.4.0] - 2021-04-08

### Features

- Add options struct for configuring behavior
- Add option for touching locally modified files
- Add option for touching ignored files
- Add option for whether to print about touch actions
- Add option to pass list of paths to operate on

## [0.3.0] - 2021-04-08

### Features

- Respect $GIT_DIR when discovering repository
- Fallback to semver metadata if no Git context
- Mask binary only dependencies behind feature flag

### Bug Fixes

- Move CLI versioning out of lib into bin only
- Only require vergen tooling for binary builds
- Correct env variable to reflect renamed app

### Chore

- [**breaking**] Demote some library functions from public to private

