extern crate core;
use crate::config;
use crate::numbers;
use crate::syscalls;
use crate::writer;
use crate::writer::*;
use crate::MyStr;
use core::fmt::{Display, Write};

#[cfg(mytest)]
use crate::bail;

pub const ERR_TOO_MANY: &'static str = "Too many arguments!";
pub const ERR_TOO_FEW: &'static str = "Too few arguments! Run without args for help.";

#[derive(Debug)]
enum InPath {
    Stdin,
    Path(MyStr),
}

#[derive(Debug)]
enum OutPath {
    Stdout,
    Path(MyStr),
}

#[derive(Debug)]
pub struct CliArgs {
    from: Option<InPath>,
    to: Option<OutPath>,
}

// impl<T: Display> Display for Option<T> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
// 	match self {
// 	    Some(t) => write!(f, "{}", t),
// 	    None => write!(f, "None")
// 	}
//     }
// }

impl Display for CliArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<MyStr> for InPath {
    fn from(value: MyStr) -> Self {
        let s = value.to_str().unwrap();
        if s == "-" {
            return Self::Stdin;
        } else if !s.starts_with('-') {
            // Maybe check if path is reasonable
            return Self::Path(value);
        } else {
            bail!(-1, "Invalid input for input path {}", s);
        }
    }
}

impl From<MyStr> for OutPath {
    fn from(value: MyStr) -> Self {
        let s = value.to_str().unwrap();
        if s == "-" {
            return Self::Stdout;
        } else if !s.starts_with('-') {
            // Maybe check if path is reasonable
            return Self::Path(value);
        } else {
            bail!(-1, "Invalid input for output path {}", s);
        };
    }
}

impl TryFrom<&[MyStr]> for CliArgs {
    type Error = WriteBuf;

    fn try_from(value: &[MyStr]) -> Result<Self, Self::Error> {
        if value.contains(&c"-h") || value.contains(&c"--help") {
            return Err(Self::usage(value[0]));
        }
        match value.len() {
            0 => {
                panic!();
            }
            1usize => {
                return Err(writer::new_str(Self::usage(value[0])));
            }
            2 => {
                return Err(writer::new_str(ERR_TOO_FEW));
            }
            3 => {
                return Ok(Self {
                    from: Some(value[1].into()),
                    to: Some(value[2].into()),
                });
            }
            _ => {
                return Err(writer::new_str(ERR_TOO_MANY));
            }
        }
    }
}

impl CliArgs {
    pub fn parse(data: &[MyStr]) -> Result<Self, <CliArgs as TryFrom<&[MyStr]>>::Error> {
        Self::try_from(data)
    }

    pub fn get_from(&self) -> Result<u32, i32> {
	if self.from.is_none() {
	    bail!(-2, "From value is None");
	}

	match &self.from.as_ref().unwrap() {
	    InPath::Stdin => {
		Ok(0)
	    },
	    InPath::Path(cstr) => {
		let fd = syscalls::open(cstr, numbers::open::READ_ONLY, 0)?;
		Ok(fd)
	    },
	}
    }

    pub fn get_to(&self) -> Result<u32, i32> {
	if self.to.is_none() {
	    bail!(-2, "to value is None");
	}

	match &self.to.as_ref().unwrap() {
	    OutPath::Stdout => {
		Ok(1)
	    },
	    OutPath::Path(cstr) => {
		let fd = syscalls::open(cstr, numbers::open::READ_ONLY, 0)?;
		Ok(fd)
	    },
	}
    }

    fn usage(us: MyStr) -> WriteBuf {
        let mut buf = writer::new();
        write!(
            buf,
            "Usage: {} [-h|--help] INPUT OUTPUT\n",
            us.to_str().unwrap()
        ).expect("usage");
        write!(buf, "\t-h --help      Show this help message\n").expect("usage");
        write!(buf, "\tINPUT          Path to file OR - for stdin\n").expect("usage");
        write!(buf, "\tOUTPUT         Path to file OR - for stdout\n\n").expect("usage");
        write!(buf, "{}\n", config::VERSION).expect("usage");

        buf
    }
}
