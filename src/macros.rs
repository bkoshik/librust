#[macro_export]
macro_rules! enum_from_display {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $($variant:ident = $value:expr => $display:expr),* $(,)+
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, Copy)]
        pub enum $name {
            $($variant = $value),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $display)),*
                }
            }
        }

        impl $name {
            pub fn from_raw(err: i64) -> Self {
                match err {
                    $($value => $name::$variant,)*
                    _ => unreachable!()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_bitflags {
    (
        pub bitflag $name_struct:ident: $type:ty {
            $(
                $flag:ident = $bit:expr
            ),* $(,)+
        }
    ) => {
        ::bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name_struct: $type {
                $(
                    const $flag = $bit;
                )*
            }
        }
    }
}

#[macro_export]
macro_rules! platform_template {
    ($name:ident) => {
        mod $name {
            mod shared {
                pub mod error;
                pub mod flags;
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
        }
    }
}
