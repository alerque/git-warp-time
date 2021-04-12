ACLOCAL_AMFLAGS = -I build-aux

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

_gwt = $(program_prefix)$(shell $(SED) -e "$(program_transform_name)" <<< git-warp-time)$(program_suffix)

licensedir = $(datarootdir)/licenses/git-warp-time

dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
git_warp_time_SOURCES = Cargo.toml build.rs src/bin/git-warp-time.rs src/lib.rs src/cli.rs
EXTRA_git_warp_time_SOURCES = Cargo.lock
bin_PROGRAMS = git-warp-time
EXTRA_DIST = build-aux/git-version-gen

BUILT_SOURCES = .version
CLEANFILES = $(BUILT_SOURCES) $(bin_PROGRAMS) $(dist_scripts_SCRIPTS)

if ENABLE_BASH_COMPLETION
bashcompletiondir = $(BASH_COMPLETION_DIR)
nodist_bashcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(_gwt)
endif

if ENABLE_FISH_COMPLETION
fishcompletiondir = $(FISH_COMPLETION_DIR)
nodist_fishcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(_gwt).fish
endif

if ENABLE_ZSH_COMPLETION
zshcompletiondir = $(ZSH_COMPLETION_DIR)
nodist_zshcompletion_DATA = $(COMPLETIONS_OUT_DIR)/_$(_gwt)
endif

if DEBUG_RELEASE
CARGO_RELEASE_ARGS=--all-features
_TARGETDIR=debug
else
CARGO_RELEASE_ARGS=--release --locked --all-features
_TARGETDIR=release
endif

export GWT_VERSION = v$(VERSION)

# Leave some tips for cargo to use so CLI knows where it is
export CONFIGURE_PREFIX = $(prefix)/
export CONFIGURE_DATADIR = $(datadir)/
export CONFIGURE_BINDIR = $(bindir)/

CARGO_VERBOSE = $(cargo_verbose_$(V))
cargo_verbose_ = $(cargo_verbose_$(AM_DEFAULT_VERBOSITY))
cargo_verbose_0 =
cargo_verbose_1 = --verbose

CARGO_TARGET = target/$(_TARGETDIR)/git-warp-time
COMPLETIONS_OUT_DIR = target/$(_TARGETDIR)/completions

git-warp-time$(EXEEXT): $(CARGO_TARGET)
	cp -bf $< $@

$(COMPLETIONS_OUT_DIR)/$(_gwt): $(CARGO_TARGET)
	cp -bf $(COMPLETIONS_OUT_DIR)/git-warp-time.bash $@

$(COMPLETIONS_OUT_DIR)/$(_gwt).elv: $(CARGO_TARGET)
	cp -bf $(COMPLETIONS_OUT_DIR)/git-warp-time.elv $@

$(COMPLETIONS_OUT_DIR)/$(_gwt).fish: $(CARGO_TARGET)
	cp -bf $(COMPLETIONS_OUT_DIR)/git-warp-time.fish $@

$(COMPLETIONS_OUT_DIR)/_$(_gwt).ps1: $(CARGO_TARGET)
	cp -bf $(COMPLETIONS_OUT_DIR)/_git-warp-time.ps1 $@

$(COMPLETIONS_OUT_DIR)/_$(_gwt): $(CARGO_TARGET)
	cp -bf $(COMPLETIONS_OUT_DIR)/_git-warp-time $@

$(CARGO_TARGET): $(git_warp_time_SOURCES)
	$(CARGO) build $(CARGO_VERBOSE) $(CARGO_RELEASE_ARGS)

.version: $(shell $(AWK) '{print ".git/" $$2}' .git/HEAD 2>/dev/null ||:)
	[ -e "$@" ] && mv "$@" "$@-prev" || touch "$@-prev"
	$(if $<,./build-aux/git-version-gen .tarball-version,printf "$(VERSION)") > "$@"
	$(CMP) -s "$@" "$@-prev" || autoreconf configure.ac --force

clean-local:
	$(CARGO) clean

check-local: cargo-test check-version

cargo-test: git-warp-time$(EXEEXT)
	$(CARGO) test --locked

check-version: git-warp-time$(EXEEXT) | .version
	./$< --version | grep -Ff $|

installcheck-local:
	./$(_gwt) version

.PHONY: lint
lint: rustfmt clippy

.PHONY: rustfmt
rustfmt:
	$(GIT) ls-files '*.rs' | $(XARGS) rustfmt --check

.PHONY: clippy
clippy:
	$(CARGO) clippy -- -D warnings

dist-hook:
	cd $(distdir)
	echo $(VERSION) > .tarball-version
