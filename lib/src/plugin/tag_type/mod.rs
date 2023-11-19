use std::any::Any;

use crate::{types::Signature, Result, io::IoHandler};

use super::PluginBase;

#[derive(Clone)]
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
    pub base: PluginBase,
    pub handler: TagTypeHandler,
}
