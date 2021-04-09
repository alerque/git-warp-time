AC_PREREQ([2.69])
AC_INIT([git-warp-time], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque])
AC_CONFIG_AUX_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-xz dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])
AC_CONFIG_MACRO_DIR([build-aux])

AC_PROG_AWK
AC_PROG_GREP
AC_PROG_SED

AC_DEFUN([AX_PROGVAR], [AC_PATH_PROG(m4_toupper($1), m4_default($2,$1))test -n "$m4_toupper($1)" || AC_MSG_ERROR([m4_default($2,$1) is required])])

AX_PROGVAR([cmp])

AC_ARG_ENABLE([dependency-checks],
              AS_HELP_STRING([--disable-dependency-checks], [Disable dependency checks]))

AS_IF([test "x$enable_dependency_checks" != "xno"], [
	AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
	AS_IF(test x$CARGO = xno,
		AC_MSG_ERROR([cargo is required.  Please install the Rust toolchain])
	)
	AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])
	AS_IF(test x$RUSTC = xno,
		AC_MSG_ERROR([rustc is required.  Please install the Rust toolchain])
	)
])

# Add --enable-debug flag to change Rust build mode
AC_ARG_ENABLE(debug,
            AS_HELP_STRING([--enable-debug],[Build Rust code with debugging information [default=no]]),
            [debug_release=$enableval],
            [debug_release=no])

AC_MSG_CHECKING(whether to build Rust code with debugging information)
if test "x$debug_release" = "xyes" ; then
    AC_MSG_RESULT(yes)
    RUST_TARGET_SUBDIR=debug
else
    AC_MSG_RESULT(no)
    RUST_TARGET_SUBDIR=release
fi
AM_CONDITIONAL([DEBUG_RELEASE], [test "x$debug_release" = "xyes"])

AC_SUBST([RUST_TARGET_SUBDIR])

AC_CONFIG_FILES([Makefile])

AC_ARG_PROGRAM

AC_OUTPUT
