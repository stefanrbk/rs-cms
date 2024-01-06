use crate::{quick_saturate_word, s15_fixed16_number_to_f64, S15Fixed16Number, D50};

use super::{Lab, XYY};

const MAX_ENCODABLE_XYZ: f64 = 1.0 + 32767.0 / 32768.0;

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

#[repr(C)]
pub struct XYZEncoded {
    pub x: u16,
    pub y: u16,
    pub z: u16,
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

    pub fn as_xyz_encoded(mut self) -> XYZEncoded {
        if self.y <= 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
        }

        let x = xyz_to_u16(self.x.clamp(0.0, MAX_ENCODABLE_XYZ));
        let y = xyz_to_u16(self.y.clamp(0.0, MAX_ENCODABLE_XYZ));
        let z = xyz_to_u16(self.z.clamp(0.0, MAX_ENCODABLE_XYZ));

        XYZEncoded { x, y, z }
    }
}

impl XYZEncoded {
    pub fn as_xyz(self) -> XYZ {
        let x = xyz_2_f64(self.x);
        let y = xyz_2_f64(self.y);
        let z = xyz_2_f64(self.z);

        XYZ { x, y, z }
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

fn xyz_to_u16(d: f64) -> u16 {
    quick_saturate_word(d * 32768.0)
}

fn xyz_2_f64(v: u16) -> f64 {
    let fix32 = (v as S15Fixed16Number) << 1;

    s15_fixed16_number_to_f64(fix32)
}
