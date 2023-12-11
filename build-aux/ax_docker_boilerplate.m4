AC_DEFUN_ONCE([AX_DOCKER_BOILERPLATE], [

        AX_TRANSFORM_PACKAGE_NAME

        AC_ARG_ENABLE([dependency-checks],
                AS_HELP_STRING([--disable-dependency-checks],
                        [Disable build tooling dependency checks]))
        AM_CONDITIONAL([DEPENDENCY_CHECKS], [test "x$enable_dependency_checks" != "xno"])

        AC_ARG_ENABLE([developer],
                AS_HELP_STRING([--enable-developer],
                        [Check for and enable tooling required only for developers]))
        AM_CONDITIONAL([DEVELOPER], [test "x$enable_developer" = "xyes"])

        AC_MSG_NOTICE([checking for tools used by automake to build Docker projects])
        AC_PROG_INSTALL
        AM_COND_IF([DEPENDENCY_CHECKS], [
                AM_COND_IF([DEVELOPER], [
                        AX_PROGVAR([docker])
                ])
        ])

        AC_CONFIG_FILES([build-aux/docker_boilerplate.mk])

])
