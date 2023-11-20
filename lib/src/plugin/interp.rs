use crate::{types::InterpFunction, Result};

use super::Plugin;

pub type InterpFnFactory =
    fn(input_chans: usize, output_chans: usize, flags: u32) -> Result<InterpFunction>;

pub struct InterpolationPlugin {
    pub base: Plugin,
    pub factory: InterpFnFactory,
}
