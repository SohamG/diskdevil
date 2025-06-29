extern crate core;
use crate::writer::*;
use crate::writer;
use crate::{CStr,MyStr};
use core::fmt::{Write,Display};
use crate::numbers::*;

#[derive(Debug)]
enum InPath{
    Stdin,
    Path(MyStr)
}


#[derive(Debug)]
enum OutPath{
    Stdout,
    Path(MyStr)
}

#[derive(Debug)]
pub struct CliArgs {
    help: bool,
    from: Option<InPath>,
    to: Option<OutPath>
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
	} else if !s.starts_with('-'){
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
	if (*value).contains(c"-h") || (*value).contains(c"--help") {
	    return Err(Self::usage());
	}
        match value.len() {
	    0 => { panic!(); },
	    1usize => {
		return Err(writer::new_str(Self::usage()));
	    },
	    2 => {
		return Err(writer::new_str("Too few arguments! Run without args for help."));
	    },
	    3 => {
		return Ok(Self {
		    help: false,
		    from: Some(value[1].into()),
		    to: Some(value[2].into()),
		});
	
	    },
	    _ => {
		return Err(writer::new_str("Too many arguments!"));
	    }

	}
    }
}

impl CliArgs {
    fn usage() -> WriteBuf {
	todo!()
    }
}
