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

    let from = 
	

    syscalls::exit(0);
}


//Required for `#![no_std]`
#[cfg(not(mytest))]
#[cfg(no_std)]
#[panic_handler]
fn mypanic(info: &core::panic::PanicInfo) -> ! {
    bail!(-100, "{}", info.message());
}

