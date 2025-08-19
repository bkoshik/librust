#![no_std]

mod platform {
    use crate::{platform_choosing, platform_template};

    mod shared {
        pub mod errno;
        mod error;
    }
    pub use shared::*;

    platform_choosing!(any(target_os = "macos"), apple);

    platform_choosing!(any(target_os = "linux"), linux);
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
