use crate::create_args;
use crate::syscall::{syscall, SyscallNumber};

#[test]
fn write_syscall() -> Result<(), crate::error::Error> {
    let args = create_args!(1, "Hello, world!\0".as_ptr() as i64, 13);

    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}
