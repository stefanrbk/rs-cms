use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, Mutex},
};

use log::Level;

use crate::{
    plugin::{
        FormatterInFactory, FormatterOutFactory, InterpFnFactory, OptimizationFn,
        ParallelizationPlugin, PluginBase, TagTypeHandler, TransformFunc,
    },
    sig, Result, MAX_CHANNELS, VERSION,
};

use super::{ErrorCode, ErrorHandlerLogFunction, Intent, Parallelization, ParametricCurve, Tag};

#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

#[derive(Clone)]
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
    parallel: Option<Parallelization>,
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
    pub fn register_interp_plugin(&mut self, data: &InterpFnFactory) -> Result<()> {
        self.interp_factory = *data;

        Ok(())
    }

    pub fn register_tag_type_plugin(&mut self, data: &TagTypeHandler) -> Result<()> {
        self.tag_types.push(data.clone());

        Ok(())
    }

    pub fn register_mpe_type_plugin(&mut self, data: &TagTypeHandler) -> Result<()> {
        self.mpe_types.push(data.clone());

        Ok(())
    }

    pub fn register_tag_plugin(&mut self, data: &Tag) -> Result<()> {
        self.tags.push(data.clone());

        Ok(())
    }

    pub fn register_formatter_plugin(&mut self, data: (&FormatterInFactory, &FormatterOutFactory)) -> Result<()> {
        self.formatters_in.push(*data.0);
        self.formatters_out.push(*data.1);

        Ok(())
    }

    pub fn register_parametric_curve_plugin(&mut self, data: &ParametricCurve) -> Result<()> {
        self.curves.push(data.clone());

        Ok(())
    }

    pub fn register_optimization_plugin(&mut self, data: &OptimizationFn) -> Result<()> {
        self.optimizations.push(*data);

        Ok(())
    }

    pub fn register_transform_plugin(&mut self, data: &TransformFunc) -> Result<()> {
        self.transforms.push(data.clone());

        Ok(())
    }

    pub fn register_parallelization_plugin(&mut self, data: &ParallelizationPlugin) -> Result<()> {
        self.parallel = Some(Parallelization {
            max_workers: data.max_workers,
            worker_flags: data.worker_flags,
            sched: data.sched,
        });

        Ok(())
    }
}
