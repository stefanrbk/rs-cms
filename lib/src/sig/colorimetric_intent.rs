use crate::types::Signature;

pub const SCENE_COLORIMETRY_ESTIMATES: Signature = Signature(0x73636F65);
pub const SCENE_APPEARANCE_ESTIMATES: Signature = Signature(0x73617065);
pub const FOCAL_PLANE_COLORIMETRY_ESTIMATES: Signature = Signature(0x66706365);
pub const REFLECTION_HARDCOPY_ORIGINAL_COLORIMETRY: Signature = Signature(0x72686F63);
pub const REFLECTION_PRINT_OUTPUT_COLORIMETRY: Signature = Signature(0x72706F63);
