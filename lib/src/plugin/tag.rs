use std::any::Any;

use crate::{types::Signature, MAX_TYPES_IN_PLUGIN};

use super::PluginBase;

pub struct TagDescriptor {
    pub elem_count: usize,
    pub n_supported_types: usize,
    pub supported_types: [Signature; MAX_TYPES_IN_PLUGIN],
    pub decide_type: Option<fn(icc_version: f64, data: &Box<dyn Any>) -> Signature>,
}

pub struct TagPlugin {
    pub base: PluginBase,
    pub sig: Signature,
    pub desc: TagDescriptor,
}
