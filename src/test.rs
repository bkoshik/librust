use alloc::boxed::Box;
use crate::create_args;
use crate::syscall::{syscall, SyscallNumber};

#[test]
fn write_syscall() -> Result<(), Box<dyn core::error::Error>> {
    let args = create_args!(1, "Hello, world!\0".as_ptr() as i64, 10);
    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}
