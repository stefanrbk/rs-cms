use crate::{plugin::IntentFn, types::Signature};

struct Intent {
    pub value: u32,
    pub desc: &'static str,
    pub r#fn: IntentFn,
}

mod context;
mod error;
pub mod plugin;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
