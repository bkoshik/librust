use crate::create_args;
use crate::error::Result;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::structs::Stat;
use crate::syscall::{syscall, SyscallNumber};

pub fn open(path: &str, open_flags: OpenFlags, permission_flags: PermissionFlags) -> Result<i64> {
    return syscall(
        SyscallNumber::Openat,
        &create_args!(
            -2,
            path.as_ptr() as i64,
            open_flags.bits() as i64,
            permission_flags.bits() as i64
        ),
    );
}

pub fn openat(
    dfd: i64,
    path: &str,
    open_flags: OpenFlags,
    permission_flags: PermissionFlags,
) -> Result<i64> {
    return syscall(
        SyscallNumber::Openat,
        &create_args!(
            dfd,
            path.as_ptr() as i64,
            open_flags.bits() as i64,
            permission_flags.bits() as i64
        ),
    );
}

pub fn read(fd: i64, buf: &mut [u8], count: usize) -> Result<i64> {
    return syscall(
        SyscallNumber::Read,
        &create_args!(fd, buf.as_ptr() as i64, count as i64),
    );
}

pub fn write(fd: i64, buf: &str, count: usize) -> Result<i64> {
    return syscall(
        SyscallNumber::Write,
        &create_args!(fd, buf.as_ptr() as i64, count as i64),
    );
}

pub fn fstat(fd: i64, stat: *mut Stat) -> Result<i64> {
    let fstat = if cfg!(target_os = "macos") {
        SyscallNumber::Fstat64
    } else {
        SyscallNumber::Fstat
    };

    return syscall(fstat, &create_args!(fd, stat as i64));
}

pub fn close(fd: i64) -> Result<i64> {
    return syscall(SyscallNumber::Close, &create_args!(-fd));
}
