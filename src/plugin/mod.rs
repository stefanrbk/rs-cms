use crate::types::Signature;

pub struct PluginBase {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
}

mod interp;
mod tag;
mod tag_type;

pub use interp::{InterpFnFactory, InterpolationPlugin};
pub use tag::TagDescriptor;
pub use tag_type::TagTypeHandler;
