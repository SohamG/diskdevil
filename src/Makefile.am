bin_PROGRAMS = diskdevil

OBJEXT=rs
diskdevil_SOURCES = main.rs
EXTRA_diskdevil_DEPENDENCIES = syscalls.rs numbers.rs writer.rs asm.rs
LDADD = main.rs

CLEANFILES=diskdevil *.d

BUILT_SOURCES = numbers.rs

AM_CFLAGS=-C relocation-model=static -C panic=abort --target x86_64-unknown-none -g # -C target-feature=-crt-static -C link-args="-nostdlib -static"

%.rs: %.cpp.rs
	$(CPP) $(CPPFLAGS)  $< -o $@
