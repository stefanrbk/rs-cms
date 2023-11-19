use crate::types::Signature;

pub const CURVE_SET: Signature = Signature(0x63767374);
pub const MATRIX: Signature = Signature(0x6D617466);
pub const CLUT: Signature = Signature(0x636C7574);

pub const BACS: Signature = Signature(0x62414353);
pub const EACS: Signature = Signature(0x65414353);

// Custom from here, not in the ICC Spec
pub const XYZ_2_LAB: Signature = Signature(0x6C327820);
pub const LAB_2_XYZ: Signature = Signature(0x78326C20);
pub const NAMED_COLOR: Signature = Signature(0x6E636C20);
pub const LAB_V2_TO_V4: Signature = Signature(0x32203420);
pub const LAB_V4_TO_V2: Signature = Signature(0x34203220);

// Identities
pub const IDENTITY: Signature = Signature(0x69646E20);

// Float to floatPCS
pub const LAB_2_FLOAT_PCS: Signature = Signature(0x64326C20);
pub const FLOAT_PCS_2_LAB: Signature = Signature(0x6C326420);
pub const XYZ_2_FLOAT_PCS: Signature = Signature(0x64327820);
pub const FLOAT_PCS_2_XYZ: Signature = Signature(0x78326420);
pub const CLIP_NEGATIVES: Signature = Signature(0x636c7020);
