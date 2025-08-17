use crate::errno::ERRNO;
use crate::error::Error;

pub fn get_errno() -> Error {
    return ERRNO.get()
}