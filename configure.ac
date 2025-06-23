#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.72])
AC_INIT([dangerdump], [0.1], [dd@sohamg.xyz])

AC_CONFIG_SRCDIR([src/main.rs])
AC_CONFIG_AUX_DIR([build-aux])
# Checks for programs.

AC_PROG_GREP
AC_ARG_VAR([RUSTC], [Location of rustc compiler])

dnl AC_MSG_CHECKING([if rustc compiler can be found])
AC_PATH_PROG([RUSTC], [rustc], [none])

AS_IF([test "x$RUSTC" = xnone], [AC_MSG_ERROR([No rustc in PATH!])])

AC_MSG_CHECKING([if sendfile system call is supported])

AS_IF([$GREP -q sendfile /proc/kallsyms], [AC_MSG_RESULT([yes])], [AC_MSG_RESULT([no])])

AC_ARG_VAR([CPP], [Location of C Pre Processor])

AC_PATH_PROG([CPP], [cpp], [no])
AS_IF([test "x$CPP" = "xno"], [AC_MSG_ERROR([No C pre processor in path!])])

AC_SUBST([rustc_sysroot], [$($RUSTC --print sysroot)])

AC_MSG_NOTICE([Setting sysroot to $rustc_sysroot])


AC_CHECK_HEADERS([asm/unistd.h], [], [AC_MSG_ERROR([Need linux headers])])

AC_CONFIG_COMMANDS([runcpp], [$CPP -P -imacros "asm/unistd.h" -o src/numbers.rs numbers.cpp.rs], [CPP=$CPP])

AC_CONFIG_FILES([rust-project.json])

AC_OUTPUT
