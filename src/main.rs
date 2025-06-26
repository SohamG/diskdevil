#![cfg_attr(not(mytest), no_std)]
#![cfg_attr(not(mytest), no_main)]
#![allow(unreachable_code)]

#[cfg(mytest)]
pub extern crate core;

#[macro_use]
pub mod writer;

use core::arch::global_asm;
use core::ffi::*;
use core::fmt::*;
use core::slice::from_raw_parts;

pub mod args;
pub mod asm;
pub mod colours;
pub mod config;
pub mod numbers;
pub mod syscalls;

pub use crate::numbers::*;

pub const MAX_ARG: usize = 10;
pub type MyStr = &'static CStr;

#[cfg(not(mytest))]
global_asm! {
    ".section .text",
    ".global _start",
    "_start:",
    "pop rdi",
    "mov rsi, rsp",
    "call main"
}

unsafe fn str_null(bytes: *const c_char) -> MyStr {
    unsafe {
        return CStr::from_ptr(bytes);
    }
}

fn file_size(fd: i32) -> u64 {
    // Hack: Get size of file by seeking to last offset, and getting the value.
    let ans = syscalls::lseek(fd.try_into().unwrap(), 0, numbers::lseek::END)
        .expect("Could not determine size of file");
    // do not forget to rewind the offset back to the start!
    syscalls::lseek(fd.try_into().unwrap(), 0, numbers::lseek::SET).unwrap();
    return ans;
	

}

#[cfg_attr(not(no_mangle), unsafe(no_mangle))]
#[cfg(not(mytest))]
pub unsafe extern "C" fn main(argc: usize, argv: *const *const c_char) -> ! {
    use args::CliArgs;

    let final_args = &unsafe {
        let argv_slice = from_raw_parts(argv, argc as usize);
        let mut ans: [MyStr; MAX_ARG] = [&(c""); MAX_ARG];
        for i in 0..argc {
            ans[i as usize] = str_null(argv_slice[i as usize]);
        }
        ans
    }[..argc];

    let cli = match CliArgs::parse(final_args) {
        Ok(c) => c,
        Err(s) => {
            syscalls::write(2, s.to_str()).unwrap();
            bail!(-1);
        }
    };

    print!("{}", cli);

    let from_fd: i32 = match cli.get_from() {
        Ok(fd) => fd.try_into().unwrap(),
        Err(e) => {
            bail!(
                -1,
                "Could not open source file {}{}{}, open returned {}",
                colours::UNDERLINE,
                cli.from,
                colours::RESET,
                e
            )
        }
    };

    let to_fd: i32 = match cli.get_to() {
        Ok(fd) => fd.try_into().unwrap(),
        Err(e) => {
            bail!(
                -1,
                "Could not open destination file {}{}{}, open returned {}",
                colours::UNDERLINE,
                cli.from,
                colours::RESET,
                e
            )
        }
    };

    let from_sz = file_size(from_fd);

    if from_sz == 0 {
	bail!(-1,
	      "{}Size of input file {}{}{}{} was zero. Perhaps it is not seekable? {}",
	      colours::FG_BRIGHT_RED,
	      colours::UNDERLINE,
	      cli.from,
	      colours::RESET,
	      colours::FG_BRIGHT_RED,
	      colours::RESET
	);
    }

    print!("Size of input file is {} bytes", from_sz);

    print!("{} {} {}", to_fd, from_fd, from_sz);
    let res = match syscalls::sendfile_all(to_fd, from_fd, from_sz) {
        Ok(b) => b,
        Err(e) => {
            bail!(
                -1,
                "{} Error: sendfile returned {} {}",
                colours::FG_BRIGHT_RED,
                e,
                colours::RESET
            );
        }
    };
    let _ = syscalls::sync();

    if res != from_sz {
        bail!(
            -1,
            "{}Short write: wrote {} fewer bytes then desired\nWrote: {}\nDesired:{} {}",
            colours::FG_BRIGHT_RED,
            from_sz - res,
            res,
            from_sz,
            colours::RESET
        );
    }

    print!(
        "{}Successfully wrote all {}{}{} bytes to {}{}{}! {}",
        colours::BOLD,
        colours::FG_BRIGHT_GREEN,
        res,
        colours::RESET,
        colours::FG_BRIGHT_BLUE,
        cli.to,
        colours::RESET,
        colours::RESET
    );

    syscalls::exit(0);
}

//Required for `#![no_std]`
#[cfg(not(mytest))]
#[cfg(no_std)] // this is to stop rust analyzer from having a menty b
#[panic_handler]
fn mypanic(info: &core::panic::PanicInfo) -> ! {
    bail!(-100, "{}", info.message());
}
