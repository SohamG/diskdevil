bin_PROGRAMS = diskdevil

diskdevil_SOURCES = main.rs syscalls.rs numbers.rs

CLEANFILES=diskdevil numbers.rs

numbers.rs: ../numbers.cpp.rs
	$(CPP) $(CPPFLAGS) -imacros "asm/unistd.h" -P $< -o $@

diskdevil$(EXEEXT): $(diskdevil_SOURCES)
	rustc --cfg no_std -C panic=abort --target x86_64-unknown-none $(RUSTCFLAGS) $< -o $@
