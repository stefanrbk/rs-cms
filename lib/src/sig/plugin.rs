use crate::types::Signature;

pub const MAGIC_NUMBER: Signature = Signature(0x61637070);

pub const MEM_HANDLER: Signature = Signature(0x6D656D48);
pub const INTERPOLATION: Signature = Signature(0x696E7048);
pub const PARAMETRIC_CURVE: Signature = Signature(0x70617248);
pub const FORMATTERS: Signature = Signature(0x66726D48);
pub const TAG_TYPE: Signature = Signature(0x74797048);
pub const TAG: Signature = Signature(0x74616748);
pub const RENDERING_INTENT: Signature = Signature(0x696E7448);
pub const MULTI_PROCESS_ELEMENT: Signature = Signature(0x6D706548);
pub const OPTIMIZATION: Signature = Signature(0x6F707448);
pub const TRANSFORM: Signature = Signature(0x7A666D48);
pub const MUTEX: Signature = Signature(0x6D747A48);
pub const PARALLELIZATION: Signature = Signature(0x70726C48);
