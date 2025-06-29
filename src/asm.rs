extern crate core;
use core::arch::asm;

pub unsafe fn movsb(src: &str, dst: &mut [u8], count: usize)
{
    unsafe {
    asm!("rep movsb",
	 in("rcx") count,
	 in("rsi") src.as_ptr(),
	 inout("rdi") dst.as_mut_ptr() =>  _);
    };
}
