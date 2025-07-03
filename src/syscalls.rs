#![cfg_attr(mytest, allow(unused_imports))]

use crate::numbers::*;
use crate::writer;
use core::arch::asm;
use core::convert::AsRef;
use core::ffi::CStr;
#[allow(unused_imports)]
use core::fmt::Write;

pub type Result = core::result::Result<u64, i32>;

pub fn write(fd: i32, data: impl AsRef<[u8]>) -> Result {
    let ptr = data.as_ref();

    let result: i64;
    unsafe {
        asm!("syscall",
             inout("rax") WRITE => result,
             in("rdi") fd,
             in("rsi") ptr.as_ptr(),
             in("rdx") ptr.len(),
             lateout("rcx") _,
             lateout("r11") _
        );
    }
    if result >= 0 {
        Ok(result as u64)
    } else {
        Err(result as i32)
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

pub fn open(path: &CStr, flags: u64, mode: i64) -> Result {
    let mut result: i64;
    unsafe {
        asm!("syscall",
             inout("rax") OPEN => result,
             in("rdi") path.as_ptr(),
             in("rsi") flags,
             in("rdx") mode,
             lateout("rcx") _,
             lateout("r11") _
        );
    };
    if result > 0 {
        return Ok(result as u64);
    } else {
        return Err(result as i32);
    }
}

pub fn sendfile(to: i32, from: i32, offset: u64, count: u64) -> Result {
    #[allow(unused_assignments)]
    let mut ans: i64 = 69696969;
    unsafe {
        asm!("syscall",
             inout("rax") SENDFILE as i64 => ans,
             in("rdi") to,
             in("rsi") from,
             in("rdx") offset,
             in("r10") if cfg!(mytest) {1024} else {count},
             lateout("rcx") _,
             lateout("r11") _
        );
    };

    if ans >= 0 {
        return Ok(ans as u64);
    } else {
        return Err(ans as i32);
    }
}

pub fn lseek(fd: i32, offset: usize, whence: u32) -> Result {
    #[allow(unused_assignments)]
    let mut ans: i64 = 69699696;

    unsafe {
        asm!("syscall",
             inout("rax") LSEEK => ans,
             in("rdi") fd,
             in("rsi") offset,
             in("rdx") whence,
             lateout("rcx") _,
             lateout("r11") _
        );
    };
    if ans >= 0 {
        return Ok(ans as u64);
    } else {
        return Err(ans as i32);
    };
}

pub fn sync() -> Result {
    unsafe {
        asm!("syscall",
             in("rax") SYNC,
             lateout("rcx") _,
             lateout("r11") _
        );
    }
    Ok(0)
}

pub fn open_str(path: &str, flags: u64, mode: i64) -> Result {
    let cs = unsafe {
        let b = writer::new_str(path);
        CStr::from_ptr(b.data.as_ptr() as *const i8)
    };
    open(cs, flags, mode)
}

pub fn sendfile_all(to: i32, from: i32, count: u64) -> Result {
    let mut sent: u64 = 0u64;
    while sent != count {
        sent += sendfile(to, from, 0, count)?;
    }
    return Ok(sent);
}
