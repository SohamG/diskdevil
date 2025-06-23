#![no_std]

use core::arch::asm;
use core::ffi::CStr;

use numbers::WRITE;

pub unsafe fn write(fd: i32, data: &[u8]) -> i32 {

    let ptr = data.as_ptr();

    let result;
    asm!("syscall",
	 inlateout("rax") WRITE => result,
	 in("rdi") fd,
	 in("rsi") ptr,
	 in("rdx") data.len(),
	 lateout("rcx") _,
	 lateout("r11") _);
    // asm!("mov rax, ${num}", "mov rdi, $1", "mov rsi, ${x}", "syscall",
    // 	 x = in(reg) ptr, out("rax") result, num = const WRITE);
    return result;

}

pub unsafe fn exit(code: i32) {
    asm!("mov rax, $60", "mov rdi, ${c:r}", "syscall", c = in(reg) code);
}
