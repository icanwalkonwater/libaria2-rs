use std::sync::atomic::AtomicBool;

pub mod actions;
pub mod download_handle;
pub mod events;
pub mod session;

pub(crate) static ARIA_STARTED: AtomicBool = AtomicBool::new(false);

pub mod errors {
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, AriaError>;

    #[derive(Debug, Error)]
    pub enum AriaError {
        #[error("Aria2 has already been started once in this process !")]
        AlreadyInitialized,
        #[error("The given handle ({0}) couldn't be used to unregister the listener, it must be invalid !")]
        InvalidCallbackHandle(usize),
        #[error("Run error code")]
        RunError(i32),
        #[error("Add error: {0}")]
        AddError(i32),
    }
}

pub mod prelude {
    pub use crate::{
        errors::Result,
        session::{Aria2Context, Session},
    };
}
