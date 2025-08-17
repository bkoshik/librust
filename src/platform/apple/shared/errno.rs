mod types {
    mod errno;
    pub use errno::*;
}
pub use types::*;

mod methods {
    mod set_errno;
    pub use set_errno::*;
    
    mod get_errno;
    pub use get_errno::*;
}
pub use methods::*;