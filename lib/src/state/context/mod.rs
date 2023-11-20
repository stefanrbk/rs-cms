use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, Mutex},
};

use log::Level;

use crate::{
    plugin::{
        FormatterInFactory, FormatterOutFactory, InterpFnFactory, OptimizationFn, Plugin,
        TagTypeHandler, TransformFunc,
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
    // pub fn new() -> Result<Self> {
    // }

    pub fn signal_error(&self, level: Level, error_code: ErrorCode, text: &str) {
        if let Some(logger) = self.0.error_logger {
            logger(&self, level, error_code, text);
        }
    }

    pub fn get_user_data(&self) -> Option<Arc<Mutex<Box<dyn Any + Sync + Send>>>> {
        Some(self.0.user_data.clone()?)
    }

    pub fn register_plugins(&self, plugins: &[&'static Plugin]) -> Result<Self> {
        let mut inner: ContextInner = self.0.deref().clone();

        inner.register_plugins(plugins);

        Ok(Context(Arc::new(inner)))
    }
}

impl ContextInner {
    pub fn register_plugins(&mut self, plugins: &[&'static Plugin]) -> Result<()> {
        for plugin in plugins {
            if plugin.magic != sig::plugin::MAGIC_NUMBER {
                return err!(self, Error, UnknownExtension, "Unrecognized plugin"; str => "Unrecognized plugin");
            }
            if plugin.expected_version > VERSION {
                return err!(
                    self,
                    Error,
                    UnknownExtension,
                    "Plugin needs version {}, current version is {}",
                    plugin.expected_version,
                    VERSION;
                    str =>
                    "Unrecognized plugin"
                );
            }
            match plugin.r#type {
                sig::plugin::INTERPOLATION => {
                    match plugin.inner.downcast_ref::<&InterpFnFactory>() {
                        Some(interp) => self.register_interp_plugin(interp)?,
                        None => {
                            return err!(str => "Interpolation plugin did not contain an InterpFnFactory")
                        }
                    }
                }
                sig::plugin::TAG_TYPE => match plugin.inner.downcast_ref::<TagTypeHandler>() {
                    Some(handler) => self.register_tag_type_plugin(handler)?,
                    None => return err!(str => "Tag type plugin did not contain a TagTypeHandler"),
                },
                sig::plugin::MULTI_PROCESS_ELEMENT => {
                    match plugin.inner.downcast_ref::<TagTypeHandler>() {
                        Some(handler) => self.register_tag_type_plugin(handler)?,
                        None => return err!(str => "MPE plugin did not contain a TagTypeHandler"),
                    }
                }
                sig::plugin::TAG => match plugin.inner.downcast_ref::<Tag>() {
                    Some(tag) => self.register_tag_plugin(tag)?,
                    None => return err!(str => "Tag plugin did not contain a Tag"),
                },
                sig::plugin::FORMATTERS => {
                    match plugin
                        .inner
                        .downcast_ref::<(&FormatterInFactory, &FormatterOutFactory)>()
                    {
                        Some(fmt) => self.register_formatter_plugin(*fmt)?,
                        None => {
                            return err!(str => "Formatter plugin did not contain a tuple of FormatterInFactory and FormatterOutFactory")
                        }
                    }
                }
                sig::plugin::OPTIMIZATION => match plugin.inner.downcast_ref::<OptimizationFn>() {
                    Some(opt) => self.register_optimization_plugin(opt)?,
                    None => {
                        return err!(str => "Optimization plugin did not contain an OptimizationFn")
                    }
                },
                sig::plugin::TRANSFORM => match plugin.inner.downcast_ref::<TransformFunc>() {
                    Some(transform) => self.register_transform_plugin(transform)?,
                    None => return err!(str => "Transform plugin did not contain a TransformFunc"),
                },
                sig::plugin::PARALLELIZATION => {
                    match plugin.inner.downcast_ref::<Parallelization>() {
                        Some(parallel) => self.register_parallelization_plugin(parallel)?,
                        None => {
                            return err!(str => "Parallelization plugin did not contain a Parallelization")
                        }
                    }
                }
                _ => {
                    return err!(self, Error, UnknownExtension, "Unrecognized plugin type '{}'", plugin.r#type; str => "Unrecognized plugin type")
                }
            }
        }

        Ok(())
    }
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

    pub fn register_formatter_plugin(
        &mut self,
        data: (&FormatterInFactory, &FormatterOutFactory),
    ) -> Result<()> {
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

    pub fn register_parallelization_plugin(&mut self, data: &Parallelization) -> Result<()> {
        self.parallel = Some(data.clone());

        Ok(())
    }
}
