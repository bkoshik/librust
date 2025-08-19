use crate::error::Error;
use core::cell::Cell;
use once_cell::sync::Lazy;
use os_thread_local::ThreadLocal;

pub static ERRNO: Lazy<ThreadLocal<Cell<Error>>> =
    Lazy::new(|| ThreadLocal::new(|| Cell::new(Error::Unknown)));
