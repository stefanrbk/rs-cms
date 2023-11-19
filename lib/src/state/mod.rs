use crate::{
    plugin::{IntentFn, TagDescriptor, ParametricCurveEvaluator, CurveDef},
    types::{Format, Pipeline, Signature},
    Result, MAX_TYPES_IN_PLUGIN,
};

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
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}

mod context;
mod error;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
