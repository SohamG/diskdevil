use core::arch::asm;
use core::convert::AsRef;

use crate::numbers::*;

pub fn write(fd: i32, data: impl AsRef<[u8]>) -> Result<u32, i32> {
    let ptr = data.as_ref();

    let result: i32;
    unsafe {
        asm!("syscall",
	 inout("rax") WRITE => result,
	 in("rdi") fd,
	 in("rsi") ptr.as_ptr(),
	 in("rdx") ptr.len(),
	 lateout("rcx") _,
	 lateout("r11") _);
    }
    if result >= 0 {
	Ok(result as u32)
    } else {
	Err(result)
    }
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

pub fn open(path: impl AsRef<[u8]>, flags: u32, mode: u32) -> Result<u32, i32> {
    let mut result: i32 = -1;
    unsafe {
	asm!("syscall",
	     inout("rax") OPEN => result,
	     in("rsi") flags,
	     in("rdx") mode,
	     lateout("rcx") _,
	     lateout("r11") _
	)
    };

    if result > 0 {
	Ok(result as u32)
    } else {
	Err(result)
    }
}
