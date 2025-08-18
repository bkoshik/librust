use crate::errno::ERRNO;
use crate::error::Error;

pub fn set_errno(err: Error) {
    ERRNO.with(|f| f.set(err));
}
