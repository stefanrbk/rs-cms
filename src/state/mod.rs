use crate::types::Signature;

pub struct PluginBase {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
}

mod context;
mod error;
pub mod plugin;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
