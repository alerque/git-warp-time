ACLOCAL_AMFLAGS = -I build-aux
AM_DISTCHECK_CONFIGURE_FLAGS = --enable-developer-mode

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)
licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = git-warp-time
git_warp_time_SOURCES = src/bin/git-warp-time.rs src/lib.rs src/cli.rs
EXTRA_git_warp_time_SOURCES =
EXTRA_DIST = Dockerfile hooks/build
dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
dist_man_MANS =

BUILT_SOURCES =

CLEANFILES =

# A classical use of the autoconf-archive include macro would expand
# INC_AMINCLUDE here, but the perl script that inlines include statements
# runs before the automake that organizes logic and performs substitution.
# Consequentially with a substitution here it becomes impossible to use
# automake conditionals and substitutions in the included Makefile fragments.
# By entering the expanded value directly we are ready in time for the inlining
# functionality and hence can use conditionals in included makefile fragments.
include $(top_srcdir)/aminclude.am

DISTCLEANFILES = @AMINCLUDE@
