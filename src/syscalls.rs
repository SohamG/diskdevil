use core::arch::asm;
use core::convert::AsRef;

use crate::numbers::*;

pub fn write(fd: i32, data: impl AsRef<[u8]>) -> i32 {
    let ptr = data.as_ref();

    let result;
    unsafe {
        asm!("syscall",
	 inlateout("rax") WRITE => result,
	 in("rdi") fd,
	 in("rsi") ptr.as_ptr(),
	 in("rdx") ptr.len(),
	 lateout("rcx") _,
	 lateout("r11") _);
    }
    return result;
}

pub fn exit(code: i32) -> ! {
    // asm!("mov rax, $60", "mov rdi, ${c:r}", "syscall", c = in(reg) code, options(noreturn));
    unsafe {
        asm!("syscall",
	 in("rax") 60,
	 in("rdi") code,
	 options(noreturn));
    }
    unreachable!()
}
