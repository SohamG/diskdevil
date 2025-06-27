bin_PROGRAMS = diskdevil
check_PROGRAMS = testdevil
OBJEXT=rs
diskdevil_SOURCES = main.rs
testdevil_SOURCES = main.rs
EXTRA_diskdevil_DEPENDENCIES = syscalls.rs numbers.rs writer.rs asm.rs
EXTRA_testdevil_DEPENDENCIES = syscalls.rs numbers.rs writer.rs asm.rs
LDADD = main.rs

CLEANFILES=diskdevil *.d

BUILT_SOURCES = numbers.rs

diskdevil_CFLAGS=--target x86_64-unknown-none -C relocation-model=static -C panic=abort -g
testdevil_CFLAGS=--cfg mytest --crate-type=bin --extern std

%.rs: %.cpp.rs
	$(CPP) $(CPPFLAGS)  $< -o $@

mostlyclean-compile:

clean-am:
	$(RM) -rf $(CLEANFILES)
