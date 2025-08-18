use crate::syscall::{syscall, SyscallNumber};

#[test]
fn write_syscall() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = [0i64; 6];
    args[0] = 1;
    args[1] = "Hello, world!\0".as_ptr() as i64;
    args[2] = 13;
    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}
