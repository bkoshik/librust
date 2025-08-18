mod shared {
    pub mod errno;
    pub mod error;
}
pub use shared::*;

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    pub mod syscall {
        mod syscall;
        pub use syscall::*;

        mod syscall_number;
        pub use syscall_number::*;
    }
}
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    pub mod syscall {
        mod syscall;
        pub use syscall::*;

        mod syscall_number;
        pub use syscall_number::*;
    }
}
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
