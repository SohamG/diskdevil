bin_PROGRAMS = diskdevil
check_PROGRAMS = testdevil.test

diskdevil_SOURCES = main.rs
testdevil_test_SOURCES = test_main.rs

EXTRA_diskdevil_DEPENDENCIES = syscalls.rs numbers.rs writer.rs asm.rs args.rs 
EXTRA_testdevil_test_DEPENDENCIES = $(EXTRA_diskdevil_DEPENDENCIES)

LDADD = main.rs
testdevil_test_LDADD = test_main.rs

CLEANFILES=diskdevil *.d numbers.rs config.rs

BUILT_SOURCES = numbers.rs config.rs

diskdevil_CFLAGS=--target x86_64-unknown-none --cfg no_std \
	-C relocation-model=static -C panic=abort -g --edition 2024 \
	--crate-type=bin -C codegen-units=1 -C lto=true -C opt-level=3
testdevil_test_CFLAGS=--cfg mytest --crate-type=bin --extern std --edition 2024 -g


TEST_LOG_DRIVER = env AM_TAP_AWK='$(AWK)' $(SHELL) $(top_srcdir)/build-aux/tap-driver.sh
TEST_LOG_DRIVER_FLAGS=--color-tests yes --no-merge 
TESTS = testdevil.test


numbers.rs: numbers.cpp.rs
	$(CPP) $(CPPFLAGS)  $< -o $@


