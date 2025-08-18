use crate::error::Error;
use core::cell::Cell;
use os_thread_local::ThreadLocal;
use once_cell::sync::Lazy;

pub static ERRNO: Lazy<ThreadLocal<Cell<Error>>> = Lazy::new(|| ThreadLocal::new(|| Cell::new(Error::Unknown)));
