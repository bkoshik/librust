use std::cell::Cell;
use crate::error::Error;

thread_local! {
    pub static ERRNO: Cell<Error> = Cell::new(Error::Unknown);
}