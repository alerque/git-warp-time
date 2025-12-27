cargo := require('cargo')
cargo-set-version := require('cargo-set-version')
gh := require('gh')
git := require('git')
gpg := require('gpg')
just := just_executable()
make := require('make')
taplo := require('taplo')

set script-interpreter := ["zsh", "+o", "nomatch", "-eu"]
set shell := ["zsh", "+o", "nomatch", "-ecu"]
set positional-arguments := true
set unstable := true

[default]
[private]
@list:
    {{ just }} --list --unsorted

nuke-n-pave:
    {{ git }} clean -dxff -e .husky -e target -e completions
    ./bootstrap.sh

dev-conf: nuke-n-pave
    ./configure --enable-developer-mode --enable-debug
    {{ make }}

rel-conf: nuke-n-pave
    ./configure --enable-developer-mode
    {{ make }}

[parallel]
build:
    {{ make }}

check:
    {{ make }} $0

lint:
    {{ make }} $0

perfect:
    {{ make }} build check lint

[doc('Block execution if Git working tree isn’t pristine.')]
[private]
pristine:
    # Make sure Git's status cache is warmed up
    {{ git }} diff --shortstat
    # Ensure there are no changes in staging
    {{ git }} diff-index --quiet --cached HEAD || exit 1
    # Ensure there are no changes in the working tree
    {{ git }} diff-files --quiet || exit 1

[doc('Block execution if we don’t have access to private keys.')]
[private]
keys:
    {{ gpg }} -a --sign > /dev/null <<< "test"

release semver: pristine keys
    {{ cargo-set-version }} set-version {{ semver }}
    {{ taplo }} format Cargo.toml
    sed -i -e "/^git-warp-time =/s/\".*\"/\"${${:-{{ semver }}}%\.*}\"/" README.md
    sed -i -e "/image:/s/:v.*/:v{{ semver }}/" action.yml
    {{ make }} SEMVER={{ semver }} CHANGELOG.md git-warp-time-{{ semver }}.md -B
    {{ git }} add Cargo.{toml,lock} README.md CHANGELOG.md action.yml
    {{ git }} commit -m "chore: Release v{{ semver }}"
    {{ git }} tag -s v{{ semver }} -F git-warp-time-{{ semver }}.md
    {{ just }} build
    {{ git }} diff-files --quiet || exit 1
    ./config.status && {{ make }}
    {{ git }} push --atomic origin master v{{ semver }}
    {{ cargo }} publish --locked

post-release semver: keys
    {{ gh }} release download v{{ semver }} --skip-existing
    ls git-warp-time-{{ semver }}.{tar.zst,zip} | xargs -n1 {{ gpg }} -a --detach-sign
    {{ gh }} release upload v{{ semver }} git-warp-time-{{ semver }}.{tar.zst,zip}.asc
