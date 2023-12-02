use crate::{types::InterpFunction, Result};

use super::Plugin;

pub type InterpFnFactory =
    fn(input_chans: usize, output_chans: usize, flags: u32) -> Result<InterpFunction>;

pub(crate) fn default_interpolators_factory(
    _in_chans: usize,
    _out_chans: usize,
    _flags: u32,
) -> Result<InterpFunction> {
    todo!()
}
