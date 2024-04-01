AC_PREREQ([2.69])
AC_INIT([git-warp-time], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-zstd dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])

AX_GIT_VERSION
AX_TRANSFORM_PACKAGE_NAME

AC_PROG_INSTALL

AX_RUST_BOILERPLATE
AX_DOCKER_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
