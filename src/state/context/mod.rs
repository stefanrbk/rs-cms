use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::MAX_CHANNELS;

use super::{plugin::{InterpolationPlugin, ParametricCurvePlugin}, ErrorHandlerLogFunction};

#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

struct ContextInner {
    alarm_codes: [u16; MAX_CHANNELS],
    adaptation_state: f64,
    user_data: Option<Arc<Mutex<Box<dyn Any + Sync + Send>>>>,
    error_logger: Option<ErrorHandlerLogFunction>,
    interp_factory: InterpolationPlugin,
    curves: Vec<ParametricCurvePlugin>,
}
