use std::any::Any;

use crate::{io::IoHandler, types::Signature, Result};

use super::Plugin;

#[derive(Clone, PartialEq, Eq)]
pub struct TagTypeHandler {
    pub sig: Signature,
    pub read: fn(
        handler: &TagTypeHandler,
        io: &mut dyn IoHandler,
        n_items: &mut usize,
        tag_size: usize,
    ) -> Result<Box<dyn Any>>,
}

pub struct TagTypePlugin {
    pub base: Plugin,
    pub handler: TagTypeHandler,
}

pub(crate) const DEFAULT_TAG_TYPE_HANDLERS: &[TagTypeHandler] = &[];
pub(crate) const DEFAULT_MPE_TYPE_HANDLERS: &[TagTypeHandler] = &[];
