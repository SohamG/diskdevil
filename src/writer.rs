extern crate core;
use crate::numbers::*;
use crate::syscalls;
use core::fmt::{Debug,Display, Write};

pub struct WriteBuf {
    pub data: [u8; MAX_PATH],
    offset: usize,
}

impl PartialEq<&str> for WriteBuf {
    fn eq(&self, other: &&str) -> bool {
	let s = core::str::from_utf8(&self.data[..self.offset]).unwrap();
	s == *other
    }
}

impl Debug for WriteBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
	<Self as Display>::fmt(&self, f)
    }
}

impl WriteBuf {
    pub fn to_str<'a>(&'a self) -> &'a str {
	return core::str::from_utf8(&self.data[..self.offset]).unwrap();
    }
}


pub fn new() -> WriteBuf {
    let data: [u8; MAX_PATH];
    unsafe {
        data = core::mem::zeroed();
    }
    let offset = 0;

    WriteBuf { data, offset }
}

pub fn new_str(s: impl Display) -> WriteBuf {
    let mut ans = new();
    write!(ans, "{}", s).unwrap();
    return ans;
}

impl core::fmt::Write for WriteBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let s_len = s.len();

        if s_len + self.offset >= MAX_PATH {
            syscalls::write(2, "String too long!").unwrap();
            return Err(core::fmt::Error);
        }

        // syscalls::write(2, "> writebuf\n");

        unsafe {
            // asm::movsb(s, &mut self.data[self.offset..], s_len);
            core::ptr::copy_nonoverlapping(
                s.as_ptr(),
                self.data[self.offset..].as_mut_ptr(),
                s_len,
            );
        }

        self.offset += s_len;

        Ok(())
    }
}

impl Display for WriteBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            core::str::from_utf8(&self.data[..self.offset]).unwrap()
        )
    }
}

#[macro_export]
macro_rules! print {
    ($f:literal $(,)? $($a:expr),*) => {
        let mut buf = crate::writer::new();

	write!(buf, $f, $( $a ),*).expect("print");
	write!(buf, "\n").expect("print");
	let result = crate::syscalls::write(1, buf.data).unwrap();
	assert!(result == buf.data.len() as u32);
    };
}


#[macro_export]
macro_rules! dbg {
    ($f:literal $(,)? $($a:expr),*) => {
        let mut buf = crate::writer::new();

	write!(buf, "Debug:{}:{}:{}: ", file!(), line!(), column!())
	    .expect("debug");
	write!(buf, $f, $( $a ),*).expect("print");
	write!(buf, "\n").expect("print");
	let result = crate::syscalls::write(2, buf.data).unwrap();
	assert!(result == buf.data.len() as u32);
    };
}

#[macro_export]
macro_rules! bail {
    ($code:expr $(,$f:literal $(,)? $($a:expr),*)?) => {
	let exit_code: i32 = $code;
	$(crate::dbg!($f, $($a),*);)?
	crate::syscalls::exit(exit_code);
    };
}

pub fn debug(s: impl Display) {
    dbg!("{}", s);
}
