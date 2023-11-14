use crate::{state::PluginBase, types::InterpFunction, Result};

pub type InterpFnFactory =
    fn(input_chans: usize, output_chans: usize, flags: u32) -> Result<InterpFunction>;

pub struct InterpolationPlugin {
    pub base: PluginBase,
    pub factory: InterpFnFactory,
}
