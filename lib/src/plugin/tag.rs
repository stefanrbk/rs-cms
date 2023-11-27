use std::any::Any;

use crate::{state::Tag, types::Signature, MAX_TYPES_IN_PLUGIN};

use super::Plugin;

#[derive(PartialEq, Eq)]
pub struct TagDescriptor {
    pub elem_count: usize,
    pub decide_type: Option<fn(icc_version: f64, data: &Box<dyn Any>) -> Signature>,
    pub supported_types: &'static [Signature],
}

pub(crate) const DEFAULT_TAGS: &[Tag] = &[];
