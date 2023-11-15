use crate::types::Signature;

pub struct PluginBase {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
}

mod formatter;
mod interp;
mod rendering_intent;
mod tag;
mod tag_type;

pub use formatter::{
    FormatterIn, FormatterIn16, FormatterInFactory, FormatterInFloat, FormatterOut, FormatterOut16,
    FormatterOutFactory, FormatterOutFloat,
};
pub use interp::{InterpFnFactory, InterpolationPlugin};
pub use tag::TagDescriptor;
pub use tag_type::TagTypeHandler;
