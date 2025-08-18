use crate::errno::get_errno;
use crate::error::Error;

impl Error {
    pub fn last() -> Self {
        return Self::from_raw(Self::last_raw());
    }

    pub fn last_raw() -> i64 {
        return get_errno() as i64;
    }
}
