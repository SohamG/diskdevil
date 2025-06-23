#![no_std]
#![no_main]

use core::ffi::CStr;
// use core::panic::PanicInfo;
use core::ptr;
use core::arch::global_asm;
use core::slice::from_raw_parts;
use core::str::from_utf8;

pub mod numbers;
mod syscalls;

global_asm!{
    ".global _start",
    "_start:",
    "pop rdi",
    "mov rsi, rsp",
    "call main"
}

unsafe fn bytes_null<'a>(bytes: *const u8) -> &'a [u8] {
    const MAX_LEN: usize = 1024;
    for i in 0..MAX_LEN {
	if *bytes.offset(i as isize) == 0 {
	    return from_raw_parts(bytes, i);
	}
    }
    return from_raw_parts(bytes, MAX_LEN);
}

#[no_mangle]
pub unsafe fn main(argc: i32, argv: *const *const u8) -> i32 {
    unsafe{
	let argv_slice = from_raw_parts(argv, argc as usize);
	// let offset = *argv as *const i8;
	syscalls::write(1, bytes_null(argv_slice[1]));
	syscalls::exit(0);
    }
    // for i in 0..argc {
    //     unsafe {
    //         let arg_ptr = *argv.offset(i as isize);
    //         if !arg_ptr.is_null() {
    //             let c_str = CStr::from_ptr(arg_ptr as *const i8);
    //             // SAFETY: assume all args are valid UTF-8 for this example
    //             if let Ok(s) = c_str.to_str() {
    //                 my_print(s);
    //                 my_print("\n");
    //             }
    //         }
    //     }
    // }

    0
}

// fn my_print(s: &str) {
//     unsafe {
//         libc::write(1, s.as_ptr() as *const _, s.len());
//     }
// }

//Required for `#![no_std]`
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
