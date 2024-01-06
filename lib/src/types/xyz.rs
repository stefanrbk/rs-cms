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
    pub fn as_xyy(self) -> XYY {
        let i_sum = 1f64 / (self.x + self.y + self.z);

        let x = self.x * i_sum;
        let y = self.y * i_sum;
        let y2 = self.y;

        XYY { x, y, y2 }
    }

    pub fn as_lab(self, whitepoint: Self) -> Lab {
        let fx = f(self.x / whitepoint.x);
        let fy = f(self.y / whitepoint.y);
        let fz = f(self.z / whitepoint.z);

        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Lab { l, a, b }
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
