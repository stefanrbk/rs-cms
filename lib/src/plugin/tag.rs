use std::any::Any;

use crate::{state::Tag, types::Signature, MAX_TYPES_IN_PLUGIN};

use super::Plugin;

#[derive(PartialEq, Eq)]
pub struct TagDescriptor<T: ?Sized> {
    pub elem_count: usize,
    pub decide_type: Option<fn(icc_version: f64, data: &Box<dyn Any>) -> Signature>,
    pub supported_types: T,
}

pub(crate) static DEFAULT_TAGS: &[Tag] = &[];
