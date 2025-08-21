#![no_std]
extern crate alloc;

mod platform {
    use crate::{platform_choosing, platform_template};

    mod shared {
        pub mod errno;
        
        mod error;
    }
    pub use shared::*;

    platform_choosing!(any(target_os = "macos"), apple);
}
pub use platform::*;

#[cfg(feature = "unix")]
pub mod unix {
    mod fs;
    pub use fs::*;
}

mod macros;

#[cfg(test)]
mod test;
