extern crate core;
use crate::config;
use crate::numbers;
use crate::syscalls;
use crate::writer;
use crate::writer::*;
use crate::MyStr;
use crate::colours;
use core::fmt::{Display, Write};

#[cfg(mytest)]
use crate::bail;

pub const ERR_TOO_MANY: &'static str = "Too many arguments!";
pub const ERR_TOO_FEW: &'static str = "Too few arguments! Run without args for help.";

#[derive(Debug)]
pub enum InPath {
    Stdin,
    Path(MyStr),
}

#[derive(Debug)]
pub enum OutPath {
    Stdout,
    Path(MyStr),
}

#[derive(Debug)]
pub struct CliArgs {
    pub from: InPath,
    pub to: OutPath,
}

impl Display for InPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
	match self {
	    InPath::Stdin => write!(f, "stdin"),
	    InPath::Path(cstr) => write!(f, "{}", cstr.to_str().unwrap())
	}
    }
}

impl Display for OutPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
	match self {
	    OutPath::Stdout => write!(f, "stdout"),
	    OutPath::Path(cstr) => write!(f, "{}", cstr.to_str().unwrap())
	}
    }
}

impl Display for CliArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
	write!(f, "👹 Writing from {}{}{} to {}{}{}...",
	       colours::FG_BRIGHT_GREEN,
	       self.from,
	       colours::RESET,
	       colours::FG_BRIGHT_RED,
	       self.to,
	       colours::RESET
	)
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
                    from: value[1].into(),
                    to: value[2].into(),
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

    pub fn get_from(&self) -> syscalls::Result {

	match &self.from {
	    InPath::Stdin => {
		Ok(0)
	    },
	    InPath::Path(cstr) => {
		let fd = syscalls::open(cstr, numbers::open::READ_ONLY, 0)?;
		Ok(fd)
	    },
	}
    }

    pub fn get_to(&self) -> syscalls::Result {

	match &self.to {
	    OutPath::Stdout => {
		Ok(1)
	    },
	    OutPath::Path(cstr) => {
		let fd = syscalls::open(cstr, numbers::open::READ_WRITE, 0666)?;
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
