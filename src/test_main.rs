use std::*;

pub use core::ffi::CStr;

pub mod asm;
pub mod numbers;
pub mod args;
pub mod config;
pub mod syscalls;
use std::fmt::{Debug,Display};

#[macro_use]
pub mod writer;


pub const MAX_ARG: usize = 10;
pub type MyStr = &'static core::ffi::CStr;

static TEST_COUNT: i32 = 9;

pub fn main() {
    println!("1..{}", TEST_COUNT);
    let res = syscalls::write(2, "test: testing write syscall!".as_bytes());

    if let Ok(r) = res {
	println!("ok 1 - Syscall write worked {r}");
    } else {
	println!("not ok 1 - Syscall write returned {}", res.unwrap_err());
    }


    let res = syscalls::write(-1, "test: testing write syscall!".as_bytes());

    if let Ok(r) = res {
	println!("not ok 2 - Syscall should have failed!");
    } else {
	println!("ok 2 - Syscall write returned expected negative value {}", res.unwrap_err());
    }

    let myargs = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-"] as &[MyStr]);

    match myargs {
	Ok(a) => {
	    println!("ok 3 - try into for cliargs works {}", a);
	},
	Err(e) => {
	    println!("not ok 3 - try into produced {}", e);
	}
    }

    let myargs2 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-h"] as &[MyStr]);

    match myargs2 {
	Ok(a) => {
	    println!("not ok 4 - cli with -h");
	},
	Err(e) => {
	    println!("ok 4 - cli with -h");
	}
    };

    let myargs3 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"kjkjkjh", c"fooo"] as &[MyStr]);

    let arg3s = "cli args too many args";
    match myargs3 {
	Ok(_) => {
	    println!("not ok 5 - returned ok {arg3s}");
	},
	Err(e) => {
	    if e == args::ERR_TOO_MANY {
		println!("ok 5 - {arg3s}");
	    } else {
		println!("not ok 5 - error msg incorrect {arg3s} {e}");
	    }
	}
    }

    let myargs3 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"bar"] as &[MyStr]);

    let arg3s = "cli args too few args";
    match myargs3 {
	Ok(_) => {
	    println!("not ok 6 - returned ok for {arg3s}");
	},
	Err(e) => {
	    if e == args::ERR_TOO_FEW {
		println!("ok 6 - {arg3s}");
	    } else {
		println!("not ok 6 - error msg incorrect for {arg3s} {e}");
	    }
	}
    }

    let t = "open normal file";
    test_result(syscalls::open(c"./main.rs", numbers::open::READ_WRITE, 0), 7, "syscall open bashrc");
    test_result(syscalls::open_str(&"./main.rs", numbers::open::READ_WRITE, 0), 8, "syscall open bashrc");

    let t = "get from fd from cli";

    let cli = TryInto::<args::CliArgs>::try_into(&[c"foo", c"-", c"-"] as &[MyStr]);


    if fail_err(&cli, 9, t) {
	let c = cli.unwrap();
	match c.get_from() {
	    Ok(0) => {
		println!("ok 9 - {t} 0");
	    },
	    Err(e) => println!("not ok 9 - {t} {e:?}"),
	    Ok(_) => println!("not ok 9 - {t} invalid Ok")
	}
    };
}




pub fn test_result<T: Debug, U: Debug>(r: Result<T, U>, num: i32, desc: &str) {
    match r {
	Ok(t) => println!("ok {num} - {desc} {t:?}"),
	Err(u) => println!("not ok {num} - {desc} {u:?}"),
    }
}

pub fn test_err<T: Debug, U: Debug>(r: Result<T, U>, num: i32, desc: &str) {
    match r {
	Ok(t) => println!("not ok {num} - {desc} {t:?}"),
	Err(u) => println!("ok {num} - {desc} {u:?}"),
    }
}

pub fn fail_err<T: Debug, U: Debug>(r: &Result<T, U>, num: i32, desc: &str) -> bool{
    match r {
	Err(e) => {
	    println!("not ok {num} - {desc} {e:?}");
	    false
	},
	_ => true
    }
}
