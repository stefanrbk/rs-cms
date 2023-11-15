use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{MAX_CHANNELS, plugin::{InterpFnFactory, TagTypeHandler}};

use super::{plugin::{ParametricCurve, Formatters, Tag}, ErrorHandlerLogFunction};

#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

struct ContextInner {
    alarm_codes: [u16; MAX_CHANNELS],
    adaptation_state: f64,
    user_data: Option<Arc<Mutex<Box<dyn Any + Sync + Send>>>>,
    error_logger: Option<ErrorHandlerLogFunction>,
    interp_factory: InterpFnFactory,
    curves: Vec<ParametricCurve>,
    formatters: Vec<Formatters>,
    tag_types: Vec<TagTypeHandler>,
    mpe_types: Vec<TagTypeHandler>,
    tags: Vec<Tag>,
}
