use std::mem::size_of;

use once_cell::sync::Lazy;

use crate::{state::Context, types::XYZ, SemVer};

pub const VERSION: SemVer = SemVer::new(0, 1, 0);

pub const D50: XYZ = XYZ {
    x: 0.9642,
    y: 1.0,
    z: 0.8249,
};

pub const MAX_CHANNELS: usize = 16;
pub const MAX_INPUT_DIMENSIONS: usize = 15;
pub const MAX_TYPES_IN_PLUGIN: usize = 20;

pub const PTR_ALIGNMENT: usize = size_of::<usize>();

pub(crate) const MAX_STAGE_CHANNELS: usize = 128;
