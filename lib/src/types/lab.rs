use std::f64::consts::PI;

use crate::{quick_saturate_word, D50};

use super::{LCh, XYZ};

const MIN_ENCODABLE_AB2: f64 = -128.0;
const MAX_ENCODABLE_AB2: f64 = (65535.0 / 256.0) - 128.0;
const MIN_ENCODABLE_AB4: f64 = -128.0;
const MAX_ENCODABLE_AB4: f64 = 127.0;

#[repr(C)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[repr(C)]
pub struct LabEncoded {
    pub l: u16,
    pub a: u16,
    pub b: u16,
}

impl Lab {
    pub fn as_xyz(self, whitepoint: XYZ) -> XYZ {
        let y = (self.l + 16.0) / 116.0;
        let x = y + 0.002 * self.a;
        let z = y - 0.005 * self.b;

        let x = f_1(x) * whitepoint.x;
        let y = f_1(y) * whitepoint.y;
        let z = f_1(z) * whitepoint.z;

        XYZ { x, y, z }
    }

    pub fn as_lab_encoded_v2(self) -> LabEncoded {
        let l = clamp_l_f64_v2(self.l);
        let a = clamp_ab_f64_v2(self.a);
        let b = clamp_ab_f64_v2(self.b);

        let l = l_to_u16_v2(l);
        let a = ab_to_u16_v2(a);
        let b = ab_to_u16_v2(b);

        LabEncoded { l, a, b }
    }

    pub fn as_lab_encoded(self) -> LabEncoded {
        let l = clamp_l_f64_v4(self.l);
        let a = clamp_ab_f64_v4(self.a);
        let b = clamp_ab_f64_v4(self.b);

        let l = l_to_u16_v4(l);
        let a = ab_to_u16_v4(a);
        let b = ab_to_u16_v4(b);

        LabEncoded { l, a, b }
    }

    pub fn as_lch(self) -> LCh {
        let l = self.l;
        let c = (sqr(self.a) + sqr(self.b)).powf(0.5);
        let h = atan_to_deg(self.b, self.a);

        LCh { l, c, h }
    }
}

impl LabEncoded {
    pub fn as_lab_v2(self) -> Lab {
        let l = l_to_f64_v2(self.l);
        let a = ab_to_f64_v2(self.a);
        let b = ab_to_f64_v2(self.b);

        Lab { l, a, b }
    }
    pub fn as_lab(self) -> Lab {
        let l = l_to_f64_v4(self.l);
        let a = ab_to_f64_v4(self.a);
        let b = ab_to_f64_v4(self.b);

        Lab { l, a, b }
    }
}

fn f_1(t: f64) -> f64 {
    const LIMIT: f64 = 24.0 / 116.0;

    if t <= LIMIT {
        (108.0 / 841.0) * (t - (16.0 / 116.0))
    } else {
        t * t * t
    }
}

fn l_to_f64_v2(v: u16) -> f64 {
    v as f64 / 652.8
}

fn ab_to_f64_v2(v: u16) -> f64 {
    (v as f64 / 256.0) - 128.0
}

fn l_to_f64_v4(v: u16) -> f64 {
    v as f64 / 655.35
}

fn ab_to_f64_v4(v: u16) -> f64 {
    (v as f64 / 257.0) - 128.0
}

fn l_to_u16_v2(l: f64) -> u16 {
    quick_saturate_word(l * 652.8)
}

fn ab_to_u16_v2(ab: f64) -> u16 {
    quick_saturate_word((ab + 128.0) * 256.0)
}

fn clamp_l_f64_v2(l: f64) -> f64 {
    const L_MAX: f64 = (0xffffu16 as f64 * 100.0) / 0xff00u16 as f64;

    l.clamp(0.0, L_MAX)
}

fn clamp_ab_f64_v2(ab: f64) -> f64 {
    ab.clamp(MIN_ENCODABLE_AB2, MAX_ENCODABLE_AB2)
}

fn clamp_l_f64_v4(l: f64) -> f64 {
    l.clamp(0.0, 100.0)
}

fn clamp_ab_f64_v4(ab: f64) -> f64 {
    ab.clamp(MIN_ENCODABLE_AB4, MAX_ENCODABLE_AB4)
}

fn l_to_u16_v4(l: f64) -> u16 {
    quick_saturate_word(l * 655.35)
}

fn ab_to_u16_v4(ab: f64) -> u16 {
    quick_saturate_word((ab + 128.0) * 257.0)
}

fn sqr(v: f64) -> f64 {
    v * v
}

fn atan_to_deg(a: f64, b: f64) -> f64 {
    let mut h = if a == 0.0 && b == 0.0 {
        0.0
    } else {
        a.atan2(b)
    } * (180.0 * PI);

    while h > 360.0 {
        h -= 360.0;
    }

    while h < 0.0 {
        h += 360.0;
    }

    h
}
