use std::mem::size_of;

use once_cell::sync::Lazy;

use crate::{SemVer, state::Context};

pub const VERSION: SemVer = SemVer::new(0, 1, 0);

pub const MAX_CHANNELS: usize = 16;
pub const MAX_INPUT_DIMENSIONS: usize = 15;
pub const MAX_TYPES_IN_PLUGIN: usize = 20;

pub const PTR_ALIGNMENT: usize = size_of::<usize>();

pub static DEFAULT_CONTEXT: Lazy<Context> = Lazy::new(|| Context::new().unwrap());
