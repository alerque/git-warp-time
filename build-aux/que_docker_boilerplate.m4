AC_DEFUN_ONCE([QUE_DOCKER_BOILERPLATE], [

        QUE_TRANSFORM_PACKAGE_NAME

        AC_ARG_ENABLE([dependency-checks],
                AS_HELP_STRING([--disable-dependency-checks],
                        [Disable build tooling dependency checks]))
        AM_CONDITIONAL([DEPENDENCY_CHECKS], [test "x$enable_dependency_checks" != "xno"])

        AC_ARG_ENABLE([developer],
                AS_HELP_STRING([--enable-developer],
                        [Check for and enable tooling required only for developers]))
        AM_CONDITIONAL([DEVELOPER_MODE], [test "x$enable_developer" = "xyes"])

        AC_MSG_NOTICE([checking for tools used by automake to build Docker projects])
        AC_PROG_INSTALL
        AM_COND_IF([DEPENDENCY_CHECKS], [
                AM_COND_IF([DEVELOPER_MODE], [
                        QUE_PROGVAR([docker])
                ])
        ])

        AC_REQUIRE([AX_AM_MACROS])
        AX_ADD_AM_MACRO([dnl
EXTRA_DIST += build-aux/que_docker_boilerplate.am

$($SED -E "s/@PACKAGE_VAR@/$PACKAGE_VAR/g" build-aux/que_docker_boilerplate.am)
])dnl

])
