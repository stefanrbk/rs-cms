use crate::{
    types::{Format, Pipeline},
    Result,
};

use super::Plugin;

pub type OptimizationFn = fn(
    lut: &mut Pipeline,
    intent: u32,
    in_format: &mut Format,
    out_format: &mut Format,
    flags: &mut u32,
) -> Result<()>;

pub(crate) static DEFAULT_OPTIMIZATIONS: &[OptimizationFn] = &[];
