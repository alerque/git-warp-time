set ignore-comments := true
set shell := ["zsh", "+o", "nomatch", "-ecu"]
set unstable := true
set script-interpreter := ["zsh", "+o", "nomatch", "-eu"]

_default:
	@just --list --unsorted

[private]
[doc('Block execution if Git working tree isn’t pristine.')]
pristine:
	# Ensure there are no changes in staging
	git diff-index --quiet --cached HEAD || exit 1
	# Ensure there are no changes in the working tree
	git diff-files --quiet || exit 1

[private]
[doc('Block execution if we don’t have access to private keys.')]
keys:
	gpg -a --sign > /dev/null <<< "test"

release semver: pristine keys
	cargo-set-version set-version {{semver}}
	taplo format Cargo.toml
	sed -i -e "/^git-warp-time =/s/\".*\"/\"${${:-{{semver}}}%\.*}\"/" README.md
	sed -i -e "/image:/s/:v.*/:v{{semver}}/" action.yml
	make SEMVER={{semver}} CHANGELOG.md git-warp-time-{{semver}}.md -B
	git add Cargo.{toml,lock} README.md CHANGELOG.md action.yml
	git commit -m "chore: Release v{{semver}}"
	git tag -s v{{semver}} -F git-warp-time-{{semver}}.md
	cargo build
	git diff-files --quiet || exit 1
	./config.status && make
	git push --atomic origin master v{{semver}}
	cargo publish --locked

post-release semver: keys
	gh release download v{{semver}}
	ls git-warp-time-{{semver}}.{tar.zst,zip} | xargs -n1 gpg -a --detach-sign
	gh release upload v{{semver}} git-warp-time-{{semver}}.{tar.zst,zip}.asc

# vim: set ft=just
