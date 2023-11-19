use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use log::Level;

use crate::{
    plugin::{FormatterInFactory, FormatterOutFactory, InterpFnFactory, TagTypeHandler},
    types::TransformFunc,
    MAX_CHANNELS,
};

use super::{ErrorCode, ErrorHandlerLogFunction, Intent, OptimizationFn, ParametricCurve, Tag};

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
    optimizations: Vec<OptimizationFn>,
    transforms: Vec<TransformFunc>,
}

impl Context {
    pub fn signal_error(&self, level: Level, error_code: ErrorCode, text: &str) {
        if let Some(logger) = self.0.error_logger {
            logger(&self, level, error_code, text);
        }
    }

    pub fn get_user_data(&self) -> Option<Arc<Mutex<Box<dyn Any + Sync + Send>>>> {
        Some(self.0.user_data.clone()?)
    }
}

impl ContextInner {
    pub fn register_interp_plugin(&mut self, data: InterpolationPlugin) -> Result<()> {
        self.interp_factory = data.factory;

        Ok(())
    }
}
