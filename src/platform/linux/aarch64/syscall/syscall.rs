use crate::error::*;
use crate::syscall::SyscallNumber;
use core::arch::asm;

pub fn syscall(num: SyscallNumber, args: &[i64; 6]) -> Result<i64> {
    let result: i64;

    unsafe {
        asm!(
            "svc #0",
            "cset {cf}, cs",
            in("x0") args[0],
            in("x1") args[1],
            in("x2") args[2],
            in("x3") args[3],
            in("x4") args[4],
            in("x5") args[5],
            in("x8") num as i64,
            lateout("x0") result,
            options(nostack),
        );
    }
    if result < 0 {
        Error::set_raw(result);
        return Err(Error::last());
    }

    return Ok(result);
}
