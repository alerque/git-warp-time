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
EXTRA_git_warp_time_SOURCES = Cargo.lock .version
bin_PROGRAMS = git-warp-time
EXTRA_DIST = build-aux/git-version-gen

BUILT_SOURCES = .version
CLEANFILES = $(bin_PROGRAMS) $(BUILT_SOURCES) .version-prev

if DEBUG_RELEASE
CARGO_RELEASE_ARGS = --all-features
else
CARGO_RELEASE_ARGS = --release --locked --all-features
endif

CARGO_ENV = CARGO_TARGET_DIR=@abs_top_builddir@/target
RUST_BIN = @abs_top_builddir@/target/@RUST_TARGET_SUBDIR@/$(PACKAGE_NAME)
_RUST_OUT = @abs_top_builddir@/target/@RUST_TARGET_SUBDIR@/.cargo_out_dir
COMPLETIONS_OUT_DIR = completions

if ENABLE_BASH_COMPLETION
bashcompletiondir = $(BASH_COMPLETION_DIR)
nodist_bashcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_bashcompletion_DATA)
endif

if ENABLE_FISH_COMPLETION
fishcompletiondir = $(FISH_COMPLETION_DIR)
nodist_fishcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish
CLEANFILES += $(nodist_fishcompletion_DATA)
endif

if ENABLE_ZSH_COMPLETION
zshcompletiondir = $(ZSH_COMPLETION_DIR)
nodist_zshcompletion_DATA = $(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_zshcompletion_DATA)
endif

export GWT_VERSION = v$(VERSION)
GIT_WARP_TIME := $(PACKAGE_NAME)$(EXEEXT)

# Leave some tips for cargo to use so CLI knows where it is
export CONFIGURE_PREFIX = $(prefix)/
export CONFIGURE_DATADIR = $(datadir)/
export CONFIGURE_BINDIR = $(bindir)/

CARGO_VERBOSE = $(cargo_verbose_$(V))
cargo_verbose_ = $(cargo_verbose_$(AM_DEFAULT_VERBOSITY))
cargo_verbose_0 =
cargo_verbose_1 = --verbose

$(COMPLETIONS_OUT_DIR):
	$(MKDIR_P) $@

$(GIT_WARP_TIME): $(RUST_BIN)
	$(INSTALL) $(RUST_BIN) $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME): $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).bash $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).elv: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).elv $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).fish $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME).ps1: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/_$(PACKAGE_NAME).ps1 $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME): $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/_$(PACKAGE_NAME) $@

$(_RUST_OUT) $(RUST_BIN): $(git_warp_time_SOURCES) $(EXTRA_git_warp_time_SOURCES)
	cd $(top_srcdir)
	$(CARGO_ENV) $(CARGO) build $(CARGO_VERBOSE) $(CARGO_RELEASE_ARGS)
	$(CARGO_ENV) $(CARGO) build --quiet --message-format=json $(CARGO_RELEASE_ARGS) |
		$(JQ) -sr 'map(select(.reason == "build-script-executed")) | last | .out_dir' > $(_RUST_OUT)

_BRANCH_REF != $(AWK) '{print ".git/" $$2}' .git/HEAD 2>/dev/null ||:

.version: $(_BRANCH_REF)
	@if [ -e "$(srcdir)/.tarball-version" ]; then \
		printf "$(VERSION)" > $@; \
	else \
		touch "$@-prev"; \
		if [ -e "$@" ]; then \
			cp "$@" "$@-prev"; \
		fi; \
		./build-aux/git-version-gen "$(srcdir)/.tarball-version" > $@; \
		$(CMP) -s "$@" "$@-prev" || autoreconf configure.ac --force; \
	fi

dist-hook:
	printf "$(VERSION)" > "$(distdir)/.tarball-version"

clean-local:
	cd $(top_srcdir)
	$(CARGO_ENV) $(CARGO) clean

check-local: cargo-test check-version

cargo-test: $(PACKAGE_NAME)$(EXEEXT)
	cd $(srcdir)
	$(CARGO_ENV) $(CARGO) test --locked

check-version: $(GIT_WARP_TIME) | .version
	./$(GIT_WARP_TIME) --version | $(GREP) -Ff .version

installcheck-local:
	./$(TRANSFORMED_PACKAGE_NAME)$(EXEEXT) --version

.PHONY: lint
lint: rustfmt clippy

.PHONY: rustfmt
rustfmt:
	$(GIT) ls-files '*.rs' | $(XARGS) rustfmt --check

.PHONY: clippy
clippy:
	cd $(srcdir)
	$(CARGO_ENV) $(CARGO) clippy -- -D warnings
