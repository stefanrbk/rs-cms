use crate::{
    plugin::{CurveDef, IntentFn, ParametricCurveEvaluator, TagDescriptor, Transform2Fn},
    types::{Format, Pipeline, Signature},
    Result, MAX_TYPES_IN_PLUGIN,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Intent {
    pub value: u32,
    pub desc: &'static str,
    pub r#fn: IntentFn,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tag {
    pub sig: Signature,
    pub desc: &'static TagDescriptor,
}

#[derive(Clone)]
pub struct ParametricCurve {
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}

#[derive(Clone)]
pub struct Parallelization {
    pub max_workers: i32,
    pub worker_flags: u32,
    pub sched: Transform2Fn,
}

mod context;
mod error;

pub use context::Context;
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
