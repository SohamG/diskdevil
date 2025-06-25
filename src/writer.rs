use asm;
use core::fmt::{Display, Write};
use numbers::*;
use syscalls;

pub struct WriteBuf {
    pub data: [u8; MAX_PATH],
    offset: usize,
}

pub fn new() -> WriteBuf {
    let data: [u8; MAX_PATH];
    unsafe{
	data = core::mem::zeroed();
    }
    let offset = 0;

    WriteBuf { data, offset }
}

impl core::fmt::Write for WriteBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let s_len = s.len();

            if s_len + self.offset >= MAX_PATH {
                syscalls::write(2, "String too long!");
                return Err(core::fmt::Error);
            }

            // syscalls::write(2, "> writebuf\n");


        unsafe {
            // asm::movsb(s, &mut self.data[self.offset..], s_len);
	    core::ptr::copy_nonoverlapping(s.as_ptr(), self.data[self.offset..].as_mut_ptr(), s_len);
        }

            self.offset += s_len;

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($f:literal $(,)? $($a:expr),*) => {
        let mut buf = crate::writer::new();
	write!(buf, $f, $( $a ),*).expect("print");
	write!(buf, "\n").expect("print");
	let result = syscalls::write(1, buf.data);
	assert!(result == buf.data.len() as i32);
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
	let result = syscalls::write(2, buf.data);
	assert!(result == buf.data.len() as i32);
    };
}

#[macro_export]
macro_rules! bail {
    ($code:expr $(,$f:literal $(,)? $($a:expr),*)?) => {
	let exit_code: i32 = $code;
	$(dbg!($f, $($a),*);)?
	syscalls::exit(exit_code);
    };
}

pub fn debug(s: impl Display) {
    dbg!("{}", s);
}

