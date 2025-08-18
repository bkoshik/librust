#![no_std]
extern crate alloc;

mod platform {
    use crate::platform_template;

    mod shared {
        mod error;
        pub mod errno;
    }
    pub use shared::*;

    #[cfg(target_os = "macos")]
    platform_template!(apple);
    #[cfg(target_os = "macos")]
    pub use apple::*;
}
pub use platform::*;

mod macros;

#[cfg(test)]
mod test;
