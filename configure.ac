AC_PREREQ([2.69])
AC_INIT([git-warp-time], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-zstd dist-zip no-dist-gzip no-dependencies color-tests])
AM_SILENT_RULES([yes])

# Load up re-usable m4 bits shared across Caleb's projects
QUE_GIT_VERSION
QUE_TRANSFORM_PACKAGE_NAME
QUE_DEVELOPER_MODE
QUE_DIST_CHECKSUMS

AM_COND_IF([DEVELOPER_MODE], [
    # Release tooling
    AX_REQUIRE_PROG([cargo-set-version])
    AX_REQUIRE_PROG([git-cliff])
    AX_REQUIRE_PROG([just])
    AX_REQUIRE_PROG([taplo])
    AX_REQUIRE_PROG([typos])
])

QUE_RUST_BOILERPLATE
QUE_DOCKER_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT

# Avoid the perceived need to run automake again when building from source
# tarballs. The assorted AM include macros assume two passes are needed to
# generate a makefile since one pass has to generate the includes and the
# second one inlines them, but we're distributing a ready-to-use version and
# hence one pass is enough. We can skip ahead and assume the Makefile template
# is ready to go an this point, avoiding the need for autotools at all.
AM_COND_IF([SOURCE_IS_DIST], [touch -r aminclude.am Makefile.in])
