pub const WRITE: i32 = __NR_write;
pub const READ: i32 = __NR_read;
pub const OPEN: i32 = __NR_open;
pub const SENDFILE: i32 = __NR_sendfile;
pub const MAX_PATH: usize = PATH_MAX;
pub const LSEEK: i32 = __NR_lseek;

pub mod open {
    pub const READ_ONLY: i64 = O_RDONLY;
    pub const WRITE_ONLY: i64 = O_WRONLY;
    pub const READ_WRITE: i64 = O_RDWR;
    pub const TRUNCATE: i64 = O_TRUNC;
}

pub mod lseek {
    pub const END: u32 = SEEK_END;
}
