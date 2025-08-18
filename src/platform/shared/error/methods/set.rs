use crate::errno::set_errno;
use crate::error::Error;

impl Error {
    pub fn set(self) {
        Self::set_raw(self as i64);
    }

    pub fn set_raw(err: i64) {
        set_errno(Error::from_raw(err));
    }
}
