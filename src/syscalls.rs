use core::arch::asm;
use core::convert::AsRef;
use core::ffi::CStr;
use crate::numbers::*;
use crate::writer;
use core::fmt::Write;

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

#[allow(unreachable_code)]
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

pub fn open(path: &CStr, flags: i64, mode: i64) -> Result<u32, i32> {
    let mut result: i32;
    use core::fmt::Write;
    unsafe {
	asm!("syscall",
	     inout("rax") OPEN => result,
	     in("rdi") path.as_ptr(),
	     in("rsi") flags,
	     in("rdx") mode
	);
    };
    dbg!("{}",result);
    if result > 0 {
	return Ok(result as u32);
    } else {
	return Err(result);
    }
}

pub fn sendfile(to: i32, from: i32, offset: i32, count: usize) -> Result<u32, i32> {
    let mut ans: i32 = 69696969;
    dbg!("to {} from {} offset {} count {}", to, from, offset, count);
    unsafe{
	asm!("syscall",
	     inout("rax") SENDFILE => ans,
	     in("rdi") to,
	     in("rsi") from,
	     in("rdx") 0,
	     in("r10") count
	);
    };

    if ans >= 0 {
	return Ok(ans as u32);
    } else  {
	return Err(ans as i32);
    }
}

pub fn open_str(path: &str, flags: i64, mode: i64) -> Result<u32, i32> {
    let cs = unsafe{
	let b = writer::new_str(path);
	CStr::from_ptr(b.data.as_ptr() as *const i8)
    };
    open(cs, flags, mode)
}



