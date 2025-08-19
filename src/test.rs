use crate::create_args;
use crate::flags::{OpenFlags, PermissionFlags};
use crate::structs::Stat;
use crate::syscall::{syscall, SyscallNumber};

#[test]
fn write_syscall() -> Result<(), crate::error::Error> {
    let args = create_args!(1, "Hello, world!\0".as_ptr() as i64, 13);

    syscall(SyscallNumber::Write, &args)?;

    Ok(())
}

#[test]
fn stat_syscall() -> Result<(), crate::error::Error> {
    let mut stat = core::mem::MaybeUninit::<Stat>::uninit();

    let fd = syscall(
        SyscallNumber::Openat,
        &create_args!(
            -2,
            "Cargo.toml\0".as_ptr() as i64,
            OpenFlags::ReadOnly.bits() as i64,
            PermissionFlags::empty().bits() as i64
        ),
    )
    .unwrap();

    #[cfg(target_os = "macos")]
    let fstat = SyscallNumber::Fstat64;
    #[cfg(not(target_os = "macos"))]
    let fstat = SyscallNumber::Fstat;
    syscall(fstat, &create_args!(fd, stat.as_mut_ptr() as i64))?;

    syscall(
        SyscallNumber::Write,
        &create_args!(1, stat.as_ptr() as i64, 5),
    )?;

    syscall(SyscallNumber::Close, &create_args!(fd))?;

    Ok(())
}
