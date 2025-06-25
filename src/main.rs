#![no_std]
#![no_main]
#![allow(unreachable_code)]

// use core::panic::PanicInfo;
use core::arch::global_asm;
use core::fmt::Write;
use core::slice::from_raw_parts;

pub mod asm;
pub mod numbers;
pub use numbers::*;

pub mod syscalls;
pub mod writer;

pub const MAX_ARG: usize = 10;

global_asm! {
    ".section .text",
    ".global _start",
    "_start:",
    "pop rdi",
    "mov rsi, rsp",
    "call main"
}

unsafe fn str_null<'a>(bytes: *const u8) -> &'a str {
    unsafe {
        for i in 0..numbers::MAX_PATH {
            if *bytes.offset(i as isize) == 0 {
                return core::str::from_utf8(from_raw_parts(bytes, i)).expect("");
            }
        }
        syscalls::write(2, "Argument longer than 1024".as_bytes());
        syscalls::exit(-1);
        unreachable!();
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: usize, argv: *const *const u8) -> ! {
    let final_args = &unsafe {
        let argv_slice = from_raw_parts(argv, argc as usize);
        let mut ans: [&str; MAX_ARG] = [&""; MAX_ARG];
        for i in 0..argc {
            ans[i as usize] = str_null(argv_slice[i as usize]);
        }
	ans
    }[..argc];

    
    writer::debug("🦀Args:");
    writer::debug(final_args[0]);
    writer::debug(final_args[1]);

    print!("This is a print!\n");
    print!("This is a {} print with args!!\n", final_args[1]);
    dbg!("This is debug lol {} {}", 6, 9);

    bail!(-1, "Testing bailing out! {} ", final_args[0]);

    syscalls::exit(0)
}


//Required for `#![no_std]`
#[panic_handler]
fn mypanic(info: &core::panic::PanicInfo) -> ! {
    writer::debug(info.message());
    loop {}
}
