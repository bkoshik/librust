#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Stat {
    pub st_dev: i32,
    pub st_ino: u64,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: i32,
    pub st_atimespec: TimeSpec,
    pub st_mtimespec: TimeSpec,
    pub st_ctimespec: TimeSpec,
    pub st_birthtimespec: TimeSpec,
    pub st_size: i64,
    pub st_blocks: i64,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u32,
    pub st_lspare: i32,
    pub st_qspare: [i64; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}