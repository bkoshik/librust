mod types {
    mod error;
    pub use error::*;

    mod result;
    pub use result::*;
}
pub use types::*;

mod methods {
    mod clear;
    mod last;
    mod set;
}
