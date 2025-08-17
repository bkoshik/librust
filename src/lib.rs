mod platform {
    #[cfg(target_os = "macos")]
    mod apple;
    #[cfg(target_os = "macos")]
    pub use apple::*;
}
pub use platform::*;

mod macros;
