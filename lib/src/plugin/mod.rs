use std::any::Any;

use crate::{sig, types::Signature, SemVer};

pub struct Plugin {
    pub magic: Signature,
    pub expected_version: SemVer,
    pub r#type: Signature,
    pub inner: &'static dyn Any,
}

pub fn create_interpolation_plugin(factory: &'static InterpFnFactory) -> Plugin {
    Plugin {
        magic: sig::plugin::MAGIC_NUMBER,
        expected_version: SemVer::new(0, 1, 0),
        r#type: sig::plugin::INTERPOLATION,
        inner: factory,
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
pub(crate) use interp::default_interpolators_factory;
pub use interp::InterpFnFactory;
pub use optimization::{OptimizationFn, OptimizationPlugin};
pub use parallel::ParallelizationPlugin;
pub use rendering_intent::IntentFn;
pub use tag::{TagDescriptor, TagPlugin};
pub use tag_type::{TagTypeHandler, TagTypePlugin};
pub use transform::{
    Transform2Factory, Transform2FactoryResult, Transform2Fn, TransformFactory,
    TransformFactoryResult, TransformFn, TransformFunc, TransformPlugin,
};
