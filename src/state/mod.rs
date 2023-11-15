use crate::{
    plugin::{IntentFn, TagDescriptor},
    types::Signature,
    MAX_TYPES_IN_PLUGIN,
};

type ParametricCurveEvaluator = fn(r#type: i32, params: [f64; 10], r: f64) -> f64;

struct Intent {
    pub value: u32,
    pub desc: &'static str,
    pub r#fn: IntentFn,
}

struct Tag {
    pub sig: Signature,
    pub desc: TagDescriptor,
}

struct ParametricCurve {
    pub n_fns: usize,
    pub fn_types: [u32; MAX_TYPES_IN_PLUGIN],
    pub param_count: [usize; MAX_TYPES_IN_PLUGIN],
    pub eval: ParametricCurveEvaluator,
}

mod context;
mod error;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
