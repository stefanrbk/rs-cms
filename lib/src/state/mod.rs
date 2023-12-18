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

#[derive(Clone, Eq)]
pub struct ParametricCurve {
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}

impl ParametricCurve {
    pub(crate) fn is_in_set(&self, r#type: i32) -> isize {
        for i in 0..self.curves.len() {
            if r#type.abs() as u32 == self.curves[i].fn_type { return i as isize }
        }

        -1
    }
}

impl PartialEq for ParametricCurve {
    fn eq(&self, other: &Self) -> bool {
        if self.curves.len() == other.curves.len() && self.eval == other.eval {
            for i in 0..self.curves.len() {
                if self.curves[i] != other.curves[i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parallelization {
    pub max_workers: i32,
    pub worker_flags: u32,
    pub sched: Transform2Fn,
}

mod context;
mod error;

pub use context::{Context, DEFAULT_CONTEXT};
pub use error::{default_error_handler_log_function, ErrorCode, ErrorHandlerLogFunction};
