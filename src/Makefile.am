bin_PROGRAMS = diskdevil

diskdevil_SOURCES = main.rs syscalls.rs numbers.rs writer.rs asm.rs

CLEANFILES=diskdevil *.d

BUILT_SOURCES = numbers.rs

MY_RUSTCFLAGS=-C relocation-model=static # -C target-feature=-crt-static -C link-args="-nostdlib -static"
%.rs: %.cpp.rs
	$(CPP) $(CPPFLAGS)  $< -o $@

diskdevil$(EXEEXT): $(diskdevil_SOURCES)
	$(RUSTC) $(MY_RUSTCFLAGS) --cfg no_std -C panic=abort --target x86_64-unknown-none -g $(RUSTCFLAGS) $< -o $@
