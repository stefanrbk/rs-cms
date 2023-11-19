use crate::types::Signature;

pub const MAGIC_NUMBER: Signature = Signature(0x61637370);
pub const LCMS_SIGNATURE: Signature = Signature(0x6c636d73);

pub const PERCEPTUAL_REFERENCE_MEDIUM_GAMUT: Signature = Signature(0x70726d67);

pub mod class;
pub mod colorspace;
pub mod platform;
pub mod tags;
pub mod technology;
pub mod types;
pub mod colorimetric_intent;
pub mod mpe_stage;
pub mod curve_segment;
pub mod response_curve;
pub mod plugin;
