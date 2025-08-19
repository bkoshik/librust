use alloc::boxed::Box;
use crate::create_args;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::structs::Stat;
use crate::syscall::{syscall, SyscallNumber};

#[test]
fn write_syscall() -> Result<(), Box<dyn core::error::Error>> {
    let args = create_args!(1, "Hello, world!\0".as_ptr() as i64, 13);

    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}

#[test]
fn stat_syscall() -> Result<(), Box<dyn core::error::Error>> {
    let mut stat = core::mem::MaybeUninit::<Stat>::uninit();

    let fd = syscall(
        SyscallNumber::Openat,
        &create_args!(-2, "Cargo.toml\0".as_ptr() as i64, OpenFlags::ReadOnly.bits() as i64, PermissionFlags::empty().bits() as i64),
    )?;

    let fstat = if cfg!(target_os = "macos") {
        SyscallNumber::Fstat64
    } else {
        SyscallNumber::Fstat
    };
    syscall(
        fstat,
        &create_args!(fd, stat.as_mut_ptr() as i64)
    )?;

    unsafe { println!("{:#?}", stat.assume_init()); }

    Ok(())
}