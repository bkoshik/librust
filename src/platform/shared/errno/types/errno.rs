use crate::error::Error;
use std::cell::Cell;

thread_local! {
    pub static ERRNO: Cell<Error> = Cell::new(Error::Unknown);
}
