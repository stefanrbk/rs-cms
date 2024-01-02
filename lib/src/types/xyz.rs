use crate::{S15Fixed16Number, D50};

use super::{Lab, XYY};

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

impl XYZ {
    pub fn into(self, whitepoint: Self) -> Lab {
        let fx = f(self.x / whitepoint.x);
        let fy = f(self.y / whitepoint.y);
        let fz = f(self.z / whitepoint.z);

        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Lab { l, a, b }
    }
}

impl From<Lab> for XYZ {
    fn from(value: Lab) -> Self {
        value.into(D50)
    }
}

impl From<XYY> for XYZ {
    fn from(value: XYY) -> Self {
        let x = (value.x / value.y) * value.y2;
        let y = value.y2;
        let z = ((1f64 - value.x - value.y) / value.y) * value.y2;

        Self { x, y, z }
    }
}

fn f(t: f64) -> f64 {
    const LIMIT: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);

    if t <= LIMIT {
        (841.0 / 108.0) * t + (16.0 / 116.0)
    } else {
        t.powf(1.0 / 3.0)
    }
}
