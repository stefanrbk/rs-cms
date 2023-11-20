use std::any::Any;

use crate::{state::Tag, types::Signature, MAX_TYPES_IN_PLUGIN};

use super::Plugin;

#[derive(Clone)]
pub struct TagDescriptor {
    pub elem_count: usize,
    pub n_supported_types: usize,
    pub supported_types: [Signature; MAX_TYPES_IN_PLUGIN],
    pub decide_type: Option<fn(icc_version: f64, data: &Box<dyn Any>) -> Signature>,
}

pub(crate) static DEFAULT_TAGS: &[Tag] = &[];
