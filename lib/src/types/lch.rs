use std::f64::consts::PI;

use super::Lab;

#[repr(C)]
pub struct LCh {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl LCh {
    pub fn as_lab(self) -> Lab {
        let h = (self.h * PI) / 180.0;

        let l = self.l;
        let (a, b) = h.sin_cos();
        let a = self.c * a;
        let b = self.c * b;

        Lab { l, a, b }
    }
}

fn radians(deg: f64) -> f64 {
    (deg * PI) / 180.0
}
