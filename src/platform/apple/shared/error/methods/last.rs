use crate::error::Error;
use std::arch::asm;

impl Error {
    pub fn last() -> Self {
        return Self::from_raw(Self::last_raw());
    }

    pub fn last_raw() -> i64 {
        let error;
        unsafe {
            asm!(
                "cmp x0, #0",
                "cneg x0, x0, lt",
                lateout("x0") error,
            )
        };

        return error;
    }
}
