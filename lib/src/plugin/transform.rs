use std::any::Any;

use crate::{
    types::{Format, Pipeline, Stride, Transform},
    Result,
};

use super::Plugin;

pub type TransformFn =
    fn(cargo: &Transform, in_buf: &[u8], out_buf: &mut [u8], size: usize, stride: usize);

pub type Transform2Fn = fn(
    cargo: &Transform,
    in_buf: &[u8],
    out_buf: &mut [u8],
    pix_per_line: usize,
    line_count: usize,
    stride: Stride,
);

pub type TransformFactory = fn(
    lut: &mut Pipeline,
    in_format: &mut Format,
    out_format: &mut Format,
    flags: &mut u32,
) -> Result<TransformFactoryResult>;

pub type Transform2Factory = fn(
    lut: &mut Pipeline,
    in_format: &mut Format,
    out_format: &mut Format,
    flags: &mut u32,
) -> Result<Transform2FactoryResult>;

pub struct TransformPlugin {
    pub base: Plugin,
    pub xform: TransformFunc,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TransformFunc {
    Factory(Transform2Factory),
    OldFactory(TransformFactory),
}

pub struct TransformFactoryResult {
    pub xform: TransformFn,
    pub data: Option<Box<dyn Any>>,
}

pub struct Transform2FactoryResult {
    pub xform: Transform2Fn,
    pub data: Option<Box<dyn Any>>,
}

pub(crate) const DEFAULT_TRANSFORM_FACTORIES: &[TransformFunc] = &[];
