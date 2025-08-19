use crate::error::*;
use crate::syscall::SyscallNumber;
use core::arch::asm;

pub fn syscall(num: SyscallNumber, args: &[i64; 6]) -> Result<i64> {
    let result: i64;

    unsafe {
        asm!(
            "syscall",
            "setc {cf}",
            in("rax") num as i64,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("r10") args[3],
            in("r8")  args[4],
            in("r9")  args[5],
            lateout("rax") result,
            options(nostack),
        );
    }
    if result < 0 {
        Error::set_raw(result);
        return Err(Error::last());
    }

    return Ok(result);
}
