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

    let cli = CliArgs::parse(final_args);

    dbg!("{:?}", cli);

    syscalls::exit(0)
}


//Required for `#![no_std]`
#[cfg(not(mytest))]
#[panic_handler]
fn mypanic(info: &core::panic::PanicInfo) -> ! {
    writer::debug(info.message());
    bail!(-100);
}

#[cfg(mytest)]
pub fn main() {
    use std::*;
    use crate::args;
    println!("1..{}", 5);
    let res = syscalls::write(2, "test: testing write syscall!".as_bytes());

    if res > 0 {
	println!("ok 1 - Syscall write worked");
    } else {
	println!("not ok 1 - Syscall write returned {}", res);
    }

    let res = syscalls::write(-1, "test: testing write syscall!".as_bytes());

    if res > 0 {
	println!("not ok 2 - Syscall should have failed!");
    } else {
	println!("ok 2 - Syscall write returned expected negative value {}", res);
    }

    let myargs = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-"] as &[MyStr]);

    match myargs {
	Ok(a) => {
	    println!("ok 3 - try into for cliargs works {}", a);
	},
	Err(e) => {
	    println!("not ok 3 - try into produced {}", e);
	}
    }

    let myargs2 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-h"] as &[MyStr]);

    match myargs2 {
	Ok(a) => {
	    println!("not ok 4 - cli with -h");
	},
	Err(e) => {
	    println!("ok 4 - cli with -h");
	}
    };

    let myargs3 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"kjkjkjh", c"fooo"] as &[MyStr]);

    let arg3s = "cli args too many args";
    match myargs3 {
	Ok(_) => {
	    println!("not ok 5 - returned ok {arg3s}");
	},
	Err(e) => {
	    if e == args::ERR_TOO_MANY {
		println!("ok 5 - {arg3s}");
	    } else {
		println!("not ok 5 - error msg incorrect {arg3s} {e}");
	    }
	}
    }

    

}

#[cfg(mytests)]
pub fn test_result<T: Debug, U: Debug>(r: Result<T, U>, num: i32, desc: String) {
    match r {
	Ok(t) => println!("ok {num} - {desc} {t}"),
	Err(u) => println!("not ok {num} - {desc} {u}"),
    }
}
