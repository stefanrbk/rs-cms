use std::any::Any;

use crate::{
    sig,
    state::{Intent, Parallelization, ParametricCurve, Tag},
    types::Signature,
    SemVer,
};

pub struct Plugin {
    pub(crate) magic: Signature,
    pub(crate) expected_version: SemVer,
    pub(crate) r#type: Signature,
    pub(crate) inner: &'static (dyn Any + Sync + Send),
}
impl Plugin {
    const fn new(r#type: Signature, inner: &'static (dyn Any + Sync + Send)) -> Self {
        Self {
            magic: sig::plugin::MAGIC_NUMBER,
            expected_version: SemVer::new(0, 1, 0),
            r#type,
            inner,
        }
    }
    pub const fn create_interpolation_plugin(factory: &'static InterpFnFactory) -> Self {
        Self::new(sig::plugin::INTERPOLATION, factory)
    }

    pub const fn create_parametric_curve_plugin(data: &'static &'static [ParametricCurve]) -> Self {
        Self::new(sig::plugin::PARAMETRIC_CURVE, data)
    }

    pub const fn create_formatter_plugin(
        data: &'static (&'static FormatterInFactory, &'static FormatterOutFactory),
    ) -> Self {
        Self::new(sig::plugin::FORMATTERS, data)
    }

    pub const fn create_tag_type_plugin(data: &'static &'static [TagTypeHandler]) -> Self {
        Self::new(sig::plugin::TAG_TYPE, data)
    }

    pub const fn create_tag_plugin(data: &'static &'static [Tag]) -> Self {
        Self::new(sig::plugin::TAG, data)
    }

    pub const fn create_intents_plugin(data: &'static &'static [Intent]) -> Self {
        Self::new(sig::plugin::RENDERING_INTENT, data)
    }

    pub const fn create_mpe_type_plugin(data: &'static &'static [TagTypeHandler]) -> Self {
        Self::new(sig::plugin::MULTI_PROCESS_ELEMENT, data)
    }

    pub const fn create_optimization_plugin(data: &'static &'static [OptimizationFn]) -> Self {
        Self::new(sig::plugin::OPTIMIZATION, data)
    }

    pub const fn create_transform_plugin(data: &'static &'static [TransformFunc]) -> Self {
        Self::new(sig::plugin::TRANSFORM, data)
    }

    pub const fn create_parallelization_plugin(data: &'static Parallelization) -> Self {
        Self::new(sig::plugin::PARALLELIZATION, data)
    }
}

mod curves;
mod formatter;
mod interp;
mod optimization;
mod parallel;
mod rendering_intent;
mod tag;
mod tag_type;
mod transform;

pub use curves::{CurveDef, ParametricCurveEvaluator};
pub use formatter::{
    FormatterIn, FormatterIn16, FormatterInFactory, FormatterInFloat, FormatterOut, FormatterOut16,
    FormatterOutFactory, FormatterOutFloat, FormatterPlugin,
};
pub use interp::InterpFnFactory;
pub use optimization::OptimizationFn;
pub use parallel::ParallelizationPlugin;
pub use rendering_intent::IntentFn;
pub use tag::TagDescriptor;
pub use tag_type::{TagTypeHandler, TagTypePlugin};
pub use transform::{
    Transform2Factory, Transform2FactoryResult, Transform2Fn, TransformFactory,
    TransformFactoryResult, TransformFn, TransformFunc, TransformPlugin,
};

pub(crate) use curves::DEFAULT_PARAMETRIC_CURVE;
pub(crate) use formatter::DEFAULT_FORMATTER_FACTORIES;
pub(crate) use interp::default_interpolators_factory;
pub(crate) use optimization::DEFAULT_OPTIMIZATIONS;
pub(crate) use rendering_intent::DEFAULT_INTENTS;
pub(crate) use tag::DEFAULT_TAGS;
pub(crate) use tag_type::{DEFAULT_MPE_TYPE_HANDLERS, DEFAULT_TAG_TYPE_HANDLERS};
pub(crate) use transform::DEFAULT_TRANSFORM_FACTORIES;
