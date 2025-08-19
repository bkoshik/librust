#![no_std]

mod platform {
    use crate::platform_template;

    mod shared {
        pub mod errno;
        mod error;
    }
    pub use shared::*;

    #[cfg(target_os = "macos")]
    platform_template!(apple);
    #[cfg(target_os = "macos")]
    pub use apple::*;

    #[cfg(target_os = "linux")]
    platform_template!(linux);
    #[cfg(target_os = "linux")]
    pub use linux::*;
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
