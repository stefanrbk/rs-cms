use std::any::Any;

use crate::{sig, state::ParametricCurve, types::Signature, SemVer};

pub struct Plugin {
    pub(crate) magic: Signature,
    pub(crate) expected_version: SemVer,
    pub(crate) r#type: Signature,
    pub(crate) inner: &'static dyn Any,
}
impl Plugin {
    fn new(r#type: Signature, inner: &'static dyn Any) -> Self {
        Self {
            magic: sig::plugin::MAGIC_NUMBER,
            expected_version: SemVer::new(0, 1, 0),
            r#type,
            inner,
        }
    }
    pub fn create_interpolation_plugin(factory: &'static InterpFnFactory) -> Self {
        Self::new(sig::plugin::INTERPOLATION, factory)
    }

    pub fn create_parametric_curve_plugin(data: &'static ParametricCurve) -> Self {
        Self::new(sig::plugin::PARAMETRIC_CURVE, data)
    }

    pub fn create_formatter_plugin(data: &'static (&'static FormatterInFactory, &'static FormatterOutFactory)) -> Self {
        Self::new(sig::plugin::PARAMETRIC_CURVE, data)
    }

    pub fn create_tag_type_plugin(data: &'static TagTypeHandler) -> Self {
        Self::new(sig::plugin::PARAMETRIC_CURVE, data)
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
