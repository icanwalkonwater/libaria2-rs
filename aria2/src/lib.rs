use std::sync::atomic::AtomicBool;

pub mod session;

pub(crate) static ARIA_STARTED: AtomicBool = AtomicBool::new(false);

pub mod errors {
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, AriaError>;

    #[derive(Debug, Error)]
    pub enum AriaError {
        #[error("Aria2 has already been started once in this process !")]
        AlreadyInitialized,
    }
}

pub mod prelude {
    pub use crate::errors::Result;
    pub use crate::session::Session;
}