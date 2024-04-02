ACLOCAL_AMFLAGS = -I build-aux

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)
docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = git-warp-time
git_warp_time_SOURCES = src/bin/git-warp-time.rs src/lib.rs src/cli.rs
EXTRA_git_warp_time_SOURCES =
EXTRA_DIST = Dockerfile hooks/build
dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
dist_man_MANS =

BUILT_SOURCES =

CLEANFILES =

include $(top_srcdir)/build-aux/dist_checksums.mk
include $(top_srcdir)/build-aux/rust_boilerplate.mk
include $(top_srcdir)/build-aux/docker_boilerplate.mk
