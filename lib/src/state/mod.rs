use crate::{
    plugin::{IntentFn, TagDescriptor, ParametricCurveEvaluator, CurveDef},
    types::{Format, Pipeline, Signature},
    Result, MAX_TYPES_IN_PLUGIN,
};

type OptimizationFn = fn(
    lut: &mut Pipeline,
    intent: u32,
    in_format: &mut Format,
    out_format: &mut Format,
    flags: &mut u32,
) -> Result<()>;

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
