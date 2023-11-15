use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{
    plugin::{FormatterInFactory, FormatterOutFactory, InterpFnFactory, TagTypeHandler},
    MAX_CHANNELS,
};

use super::{
    plugin::{ParametricCurve, Tag},
    ErrorHandlerLogFunction, Intent,
};

#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

struct ContextInner {
    alarm_codes: [u16; MAX_CHANNELS],
    adaptation_state: f64,
    user_data: Option<Arc<Mutex<Box<dyn Any + Sync + Send>>>>,
    error_logger: Option<ErrorHandlerLogFunction>,
    interp_factory: InterpFnFactory,
    curves: Vec<ParametricCurve>,
    formatters_in: Vec<FormatterInFactory>,
    formatters_out: Vec<FormatterOutFactory>,
    tag_types: Vec<TagTypeHandler>,
    mpe_types: Vec<TagTypeHandler>,
    tags: Vec<Tag>,
    intents: Vec<Intent>,
}
