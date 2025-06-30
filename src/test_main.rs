use std::*;

pub use core::ffi::CStr;

pub mod args;
pub mod asm;
pub mod config;
pub mod numbers;
pub mod syscalls;
use std::fmt::{Debug, Display};

#[macro_use]
pub mod writer;

pub const MAX_ARG: usize = 10;
pub type MyStr = &'static core::ffi::CStr;

pub fn main() {
    let mut count = 0;
    let res = syscalls::write(2, "test: testing write syscall!".as_bytes());
    count += 1;

    if let Ok(r) = res {
        println!("ok {count} - Syscall write worked {r}");
    } else {
        println!("not ok {count} - Syscall write returned {}", res.unwrap_err());
    }

    let res = syscalls::write(-1, "test: testing write syscall!".as_bytes());
    count += 1;

    if let Ok(r) = res {
        println!("not ok {count} - Syscall should have failed!");
    } else {
        println!(
            "ok {count} - Syscall write returned expected negative value {}",
            res.unwrap_err()
        );
    }

    let myargs = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-"] as &[MyStr]);
    count += 1;

    match myargs {
        Ok(a) => {
            println!("ok {count} - try into for cliargs works {}", a);
        }
        Err(e) => {
            println!("not ok {count} - try into produced {}", e);
        }
    }

    let myargs2 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"-h"] as &[MyStr]);

    count += 1;

    match myargs2 {
        Ok(a) => {
            println!("not ok {count} - cli with -h");
        }
        Err(e) => {
            println!("ok {count} - cli with -h");
        }
    };

    let myargs3 =
        TryInto::<args::CliArgs>::try_into(&[c"foo", c"/tmp", c"kjkjkjh", c"fooo"] as &[MyStr]);
    count += 1;

    let arg3s = "cli args too many args";
    match myargs3 {
        Ok(_) => {
            println!("not ok {count} - returned ok {arg3s}");
        }
        Err(e) => {
            if e == args::ERR_TOO_MANY {
                println!("ok {count} - {arg3s}");
            } else {
                println!("not ok {count} - error msg incorrect {arg3s} {e}");
            }
        }
    }

    let myargs3 = TryInto::<args::CliArgs>::try_into(&[c"foo", c"bar"] as &[MyStr]);
    count += 1;

    let arg3s = "cli args too few args";
    match myargs3 {
        Ok(_) => {
            println!("not ok {count} - returned ok for {arg3s}");
        }
        Err(e) => {
            if e == args::ERR_TOO_FEW {
                println!("ok {count} - {arg3s}");
            } else {
                println!("not ok {count} - error msg incorrect for {arg3s} {e}");
            }
        }
    }

    let t = "open normal file";
    count += 1;
    test_result(
        syscalls::open(c"./main.rs", numbers::open::READ_WRITE, 0),
        count,
        "syscall open bashrc",
    );
    count += 1;
    test_result(
        syscalls::open_str(&"./main.rs", numbers::open::READ_WRITE, 0),
        count,
        "syscall open bashrc",
    );

    let t = "get from fd from cli";
    count += 1;

    let cli = TryInto::<args::CliArgs>::try_into(&[c"foo", c"-", c"-"] as &[MyStr]);

    if fail_err(&cli, count, t) {
        let c = cli.unwrap();
        match c.get_from() {
            Ok(0) => {
                println!("ok {count} - {t} 0");
            }
            Err(e) => println!("not ok {count} - {t} {e:?}"),
            Ok(_) => println!("not ok {count} - {t} invalid Ok"),
        }
    };

    println!("1..{}", count);
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

pub fn fail_err<T: Debug, U: Debug>(r: &Result<T, U>, num: i32, desc: &str) -> bool {
    match r {
        Err(e) => {
            println!("not ok {num} - {desc} {e:?}");
            false
        }
        _ => true,
    }
}
