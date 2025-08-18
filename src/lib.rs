mod platform {
    #[cfg(target_os = "macos")]
    mod apple;
    #[cfg(target_os = "macos")]
    pub use apple::*;

    #[cfg(target_os = "linux")]
    mod linux;
    #[cfg(target_os = "linux")]
    pub use linux::*;
}
pub use platform::*;

mod macros;

#[cfg(test)]
mod test;
