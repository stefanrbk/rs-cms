use crate::{types::InterpFunction, Result};

use super::PluginBase;

pub type InterpFnFactory =
    fn(input_chans: usize, output_chans: usize, flags: u32) -> Result<InterpFunction>;

pub struct InterpolationPlugin {
    pub base: PluginBase,
    pub factory: InterpFnFactory,
}
