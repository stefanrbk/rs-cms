use crate::S15Fixed16Number;

use super::XYY;

#[repr(C)]
pub struct XYZNumber {
    pub x: S15Fixed16Number,
    pub y: S15Fixed16Number,
    pub z: S15Fixed16Number,
}

#[repr(C)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<XYY> for XYZ {
    fn from(value: XYY) -> Self {
        let x = (value.x / value.y) * value.y2;
        let y = value.y2;
        let z = ((1f64 - value.x - value.y) / value.y) * value.y2;

        Self { x, y, z }
    }
}
