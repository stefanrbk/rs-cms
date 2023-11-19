use crate::types::Signature;

pub struct PluginBase {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
}

mod curves;
mod formatter;
mod interp;
mod optimization;
mod rendering_intent;
mod tag;
mod tag_type;
mod transform;

pub use curves::{CurveDef, ParametricCurveEvaluator};
pub use formatter::{
    FormatterIn, FormatterIn16, FormatterInFactory, FormatterInFloat, FormatterOut, FormatterOut16,
    FormatterOutFactory, FormatterOutFloat, FormatterPlugin,
};
pub use interp::{InterpFnFactory, InterpolationPlugin};
pub use optimization::{OptimizationFn, OptimizationPlugin};
pub use rendering_intent::IntentFn;
pub use tag::{TagDescriptor, TagPlugin};
pub use tag_type::{TagTypeHandler, TagTypePlugin};
pub use transform::{
    Transform2Factory, Transform2FactoryResult, Transform2Fn, TransformFactory,
    TransformFactoryResult, TransformFn, TransformFunc, TransformPlugin,
};
