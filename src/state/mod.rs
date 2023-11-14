use crate::types::Signature;

mod context;
mod error;
pub mod plugin;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
