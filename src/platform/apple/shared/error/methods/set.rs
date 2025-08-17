use std::arch::asm;
use crate::error::Error;
use crate::errno::set_errno;

impl Error {
    pub fn set(self) {
        Self::set_raw(self as i64);
    }

    pub fn set_raw(err: i64) {
        set_errno(Error::from_raw(err));
    }
}
