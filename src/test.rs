use crate::syscall::{syscall, SyscallNumber};
use crate::flags;

#[test]
fn write_syscall() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = [0i64; 6];
    args[0] = 1;
    args[1] = "Hello, world!".as_ptr() as i64;
    args[2] = 100;
    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}

#[test]
fn open_syscall() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = [0i64; 6];
    args[0] = -2;
    args[1] = "hello\0".as_ptr() as i64;
    args[2] = 0;
    args[3] = 0;
    syscall(SyscallNumber::Openat, &args)?;

    Ok(())
}
