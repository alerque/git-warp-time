ACLOCAL_AMFLAGS = -I build-aux

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)
docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)

dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
git_warp_time_SOURCES = Cargo.toml build.rs src/bin/git-warp-time.rs src/lib.rs src/cli.rs
EXTRA_git_warp_time_SOURCES = Cargo.lock
bin_PROGRAMS = git-warp-time
EXTRA_DIST = build-aux/git-version-gen

BUILT_SOURCES = .version .cargo_out_dir
CLEANFILES = $(BUILT_SOURCES) $(bin_PROGRAMS) $(dist_scripts_SCRIPTS)

if DEBUG_RELEASE
CARGO_RELEASE_ARGS=--all-features
_TARGETDIR=debug
else
CARGO_RELEASE_ARGS=--release --locked --all-features
_TARGETDIR=release
endif

CARGO_TARGET = target/$(_TARGETDIR)/$(PACKAGE_NAME)
COMPLETIONS_OUT_DIR = completions

if ENABLE_BASH_COMPLETION
bashcompletiondir = $(BASH_COMPLETION_DIR)
nodist_bashcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME)
endif

if ENABLE_FISH_COMPLETION
fishcompletiondir = $(FISH_COMPLETION_DIR)
nodist_fishcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish
endif

if ENABLE_ZSH_COMPLETION
zshcompletiondir = $(ZSH_COMPLETION_DIR)
nodist_zshcompletion_DATA = $(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME)
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

$(PACKAGE_NAME)$(EXEEXT): $(CARGO_TARGET)
	cp -bf $< $@

$(COMPLETIONS_OUT_DIR):
	mkdir -p $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME): $(CARGO_TARGET) | .cargo_out_dir $(COMPLETIONS_OUT_DIR)
	cp -bf $$(cat .cargo_out_dir)/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).bash $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).elv: $(CARGO_TARGET) | .cargo_out_dir $(COMPLETIONS_OUT_DIR)
	cp -bf $$(cat .cargo_out_dir)/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).elv $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish: $(CARGO_TARGET) | .cargo_out_dir $(COMPLETIONS_OUT_DIR)
	cp -bf $$(cat .cargo_out_dir)/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).fish $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME).ps1: $(CARGO_TARGET) | .cargo_out_dir $(COMPLETIONS_OUT_DIR)
	cp -bf $$(cat .cargo_out_dir)/$(COMPLETIONS_OUT_DIR)/_git-warp-time.ps1 $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME): $(CARGO_TARGET) | .cargo_out_dir $(COMPLETIONS_OUT_DIR)
	cp -bf $$(cat .cargo_out_dir)/$(COMPLETIONS_OUT_DIR)/_git-warp-time $@

.cargo_out_dir $(CARGO_TARGET): $(git_warp_time_SOURCES)
	$(CARGO) build $(CARGO_VERBOSE) $(CARGO_RELEASE_ARGS)
	$(CARGO) build --quiet --message-format=json $(CARGO_RELEASE_ARGS) |
		$(JQ) -sr 'map(select(.reason == "build-script-executed")) | last | .out_dir' > .cargo_out_dir

.version: $(shell $(AWK) '{print ".git/" $$2}' .git/HEAD 2>/dev/null ||:)
	[ -e "$@" ] && mv "$@" "$@-prev" || touch "$@-prev"
	$(if $<,./build-aux/git-version-gen .tarball-version,printf "$(VERSION)") > "$@"
	$(CMP) -s "$@" "$@-prev" || autoreconf configure.ac --force

clean-local:
	$(CARGO) clean

check-local: cargo-test check-version

cargo-test: $(PACKAGE_NAME)$(EXEEXT)
	$(CARGO) test --locked

check-version: $(PACKAGE_NAME)$(EXEEXT) | .version
	./$< --version | grep -Ff $|

installcheck-local:
	./$(TRANSFORMED_PACKAGE_NAME) version

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
