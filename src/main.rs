#![cfg_attr(not(mytest), no_std)]
#![cfg_attr(not(mytest), no_main)]
#![allow(unreachable_code)]

#[cfg(mytest)]
pub extern crate core;

#[macro_use]
pub mod writer;

use core::arch::global_asm;
use core::slice::from_raw_parts;
use core::ffi::*;
use core::fmt::*;

pub mod asm;
pub mod numbers;
pub mod args;
pub mod config;
pub mod syscalls;
pub mod colours;

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
    unsafe{
	return CStr::from_ptr(bytes);
    }
}

fn file_size(fd: u64) -> u64 {
    syscalls::lseek(fd.try_into().unwrap(), 0, numbers::lseek::END)
	.expect("Could not determine size of file")
}

#[cfg_attr(not(no_mangle), unsafe(no_mangle))]
#[cfg(not(mytest))]
pub unsafe extern "C" fn main(argc: usize, argv:  *const *const c_char) -> ! {
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

    let from_fd = match cli.get_from() {
        Ok(fd) => fd,
        Err(e) => {
	    bail!(-1, "Could not open source file {}{}{}, open returned {}",
		  colours::UNDERLINE,
		  cli.from,
		  colours::RESET,
		  e)
	},
    };

    let to_fd = match cli.get_to() {
        Ok(fd) => fd,
        Err(e) => {
	    bail!(-1, "Could not open destination file {}{}{}, open returned {}",
		  colours::UNDERLINE,
		  cli.from,
		  colours::RESET,
		  e)
	},
    };

    let from_sz = file_size(from_fd);

    print!("Size of input file is {} bytes", from_sz);

    // let res = syscalls::sendfile_all(to_fd, from_fd, count)
    

    
    
	

    syscalls::exit(0);
}


//Required for `#![no_std]`
#[cfg(not(mytest))]
#[cfg(no_std)] // this is to stop rust analyzer from having a menty b
#[panic_handler]
fn mypanic(info: &core::panic::PanicInfo) -> ! {
    bail!(-100, "{}", info.message());
}

