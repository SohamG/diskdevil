#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.72])
AC_INIT([diskdevil], [0.1], [dd@sohamg.xyz])

AC_CONFIG_SRCDIR([src/main.rs])
AC_CONFIG_AUX_DIR([build-aux])

AM_INIT_AUTOMAKE([foreign no-dependencies nostdinc -Wall])
AC_REQUIRE_AUX_FILE([tap-driver.pl])
# Checks for programs.

AC_PROG_GREP
AC_ARG_VAR([RUSTC], [Location of rustc compiler])

dnl AC_MSG_CHECKING([if rustc compiler can be found])
AC_PATH_PROG([RUSTC], [rustc], [none])

AS_IF([test "x$RUSTC" = xnone], [AC_MSG_ERROR([No rustc in PATH!])])

CC="$RUSTC"
AC_SUBST([CC])

AC_MSG_CHECKING([if sendfile system call is supported])

AS_IF([$GREP -q sendfile /proc/kallsyms], [AC_MSG_RESULT([yes])], [AC_MSG_RESULT([no])])

AC_ARG_VAR([CPP], [Location of C Pre Processor])

AC_PATH_PROG([CPP], [cpp], [no])
AS_IF([test "x$CPP" = "xno"], [AC_MSG_ERROR([No C pre processor in path!])])

AC_SUBST([rustc_sysroot], [$($RUSTC --print sysroot)])

AC_MSG_NOTICE([Setting sysroot to $rustc_sysroot])

AC_ARG_VAR([RUSTCFLAGS], [Additional rustc flags flags])
CFLAGS="$RUSTCFLAGS"

AC_SUBST([CFLAGS])

AC_MSG_CHECKING([for presence of asm/unistd.h])


AS_IF([$CPP $CPPFLAGS -imacros "asm/unistd.h" -imacros "fcntl.h" /dev/null 2>&1 > /dev/null],
      [AC_MSG_RESULT([yes])],
      [AC_MSG_ERROR([header asm/unistd.h not found!])])

AC_MSG_CHECKING([for presence of limits.h])


AS_IF([$CPP $CPPFLAGS -imacros "limits.h" /dev/null 2>&1 > /dev/null],
      [AC_MSG_RESULT([yes])],
      [AC_MSG_ERROR([header limits.h not found!])])

CPPFLAGS="$CPPFLAGS -imacros asm/unistd.h -imacros limits.h -P"

AC_ARG_VAR([CPPFLAGS], [C Pre Processor flags])

AC_CONFIG_COMMANDS([cpp], [$CPP $CPPFLAGS src/numbers.cpp.rs -o src/numbers.rs], [CPP="$CPP" CPPFLAGS="$CPPFLAGS"])

AC_CONFIG_FILES([rust-project.json Makefile src/Makefile tests/Makefile])

AC_OUTPUT
