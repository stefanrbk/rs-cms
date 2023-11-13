use crate::S15Fixed16Number;

#[repr(C)]
pub struct XYZNumber {
    pub x: S15Fixed16Number,
    pub y: S15Fixed16Number,
    pub z: S15Fixed16Number,
}
