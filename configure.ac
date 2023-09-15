AC_PREREQ([2.69])
AC_INIT([git-warp-time], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-xz dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])

AC_PROG_AWK
AC_PROG_GREP
AC_PROG_INSTALL

AX_PROGVAR([cmp])

AX_RUST_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
