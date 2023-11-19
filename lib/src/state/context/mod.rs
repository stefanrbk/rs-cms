use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use log::Level;

use crate::{
    plugin::{FormatterInFactory, FormatterOutFactory, InterpFnFactory, TagTypeHandler, InterpolationPlugin, TagTypePlugin, TagPlugin, FormatterPlugin, OptimizationFn, OptimizationPlugin},
    types::TransformFunc,
    MAX_CHANNELS, Result,
};

use super::{ErrorCode, ErrorHandlerLogFunction, Intent, ParametricCurve, Tag};

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
    pub fn register_interp_plugin(&mut self, data: &InterpolationPlugin) -> Result<()> {
        self.interp_factory = data.factory;

        Ok(())
    }

    pub fn register_tag_type_plugin(&mut self, data: &TagTypePlugin) -> Result<()> {
        self.tag_types.push(data.handler.clone());

        Ok(())
    }

    pub fn register_mpe_type_plugin(&mut self, data: &TagTypePlugin) -> Result<()> {
        self.mpe_types.push(data.handler.clone());

        Ok(())
    }

    pub fn register_tag_plugin(&mut self, data: &TagPlugin) -> Result<()> {
        self.tags.push(Tag { sig: data.sig, desc: data.desc.clone() });

        Ok(())
    }

    pub fn register_formatter_plugin(&mut self, data: &FormatterPlugin) -> Result<()> {
        self.formatters_in.push(data.in_factory);
        self.formatters_out.push(data.out_factory);

        Ok(())
    }

    pub fn register_parametric_curve_plugin(&mut self, data: &FormatterPlugin) -> Result<()> {
        self.formatters_in.push(data.in_factory);
        self.formatters_out.push(data.out_factory);

        Ok(())
    }

    pub fn register_optimization_plugin(&mut self, data: &OptimizationPlugin) -> Result<()> {
        self.optimizations.push(data.r#fn);

        Ok(())
    }
}
