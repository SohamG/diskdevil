pub const WRITE: i64        = __NR_write;
pub const READ: i64         = __NR_read;
pub const OPEN: i64         = __NR_open;
pub const SENDFILE: i64     = __NR_sendfile;
pub const MAX_PATH: usize   = PATH_MAX;
pub const LSEEK: i64        = __NR_lseek;
pub const SYNC: i64         = __NR_sync;

pub mod open {
    pub const READ_ONLY: u64  = O_RDONLY;
    pub const WRITE_ONLY: u64 = O_WRONLY;
    pub const READ_WRITE: u64 = O_RDWR;
    pub const TRUNCATE: u64   = O_TRUNC;
    pub const CREATE: u64     = O_CREAT;
}

pub mod lseek {
    pub const END: u32 = SEEK_END;
    pub const SET: u32 = SEEK_SET;
}
