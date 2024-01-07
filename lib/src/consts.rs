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

pub(crate) const MATRIX_DET_TOLERANCE: f64 = 1e-4f64;
pub(crate) const MAX_NODES_IN_CURVE: usize = 4097;
pub(crate) const MINUS_INF: f64 = -1e22f64;
pub(crate) const PLUS_INF: f64 = 1e22f64;

pub const SAMPLER_INSPECT: u32 = 0x01000000;
pub mod flags {
    pub const NO_CACHE: u32 = 0x0040;
    pub const NO_OPTIMIZE: u32 = 0x0100;
    pub const NULL_TRANSFORM: u32 = 0x0200;
    pub const GAMUT_CHECK: u32 = 0x1000;
    pub const SOFT_PROOFING: u32 = 0x4000;
    pub const BLACK_POINT_COMPENSATION: u32 = 0x2000;
    pub const NO_WHITE_ON_WHITE_FIXUP: u32 = 0x0004;
    pub const HIGH_RES_PRECALC: u32 = 0x0400;
    pub const LOW_RES_PRECALC: u32 = 0x0800;
    pub const BITS_8_DEVICE_LINK: u32 = 0x0008;
    pub const GUESS_DEVICE_CLASS: u32 = 0x0020;
    pub const KEEP_SEQUENCE: u32 = 0x0080;
    pub const FORCE_CLUT: u32 = 0x0002;
    pub const CLUT_POST_LINEARIZATION: u32 = 0x0001;
    pub const CLUT_PRE_LINEARIZATION: u32 = 0x0010;
    pub const NO_NEGATIVES: u32 = 0x8000;
    pub const COPY_ALPHA: u32 = 0x04000000;
    pub const NO_DEFAULT_RESOURCE_DEF: u32 = 0x01000000;
    pub const fn grid_points(n: usize) -> usize {
        (n & 0xff) << 16
    }
}
