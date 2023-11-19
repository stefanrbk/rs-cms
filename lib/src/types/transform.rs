use std::any::Any;

use crate::Result;

use super::{Format, Pipeline};

pub struct Transform {}

pub struct Stride {
    pub per_line_in: usize,
    pub per_line_out: usize,
    pub per_plane_in: usize,
    pub per_plane_out: usize,
}
