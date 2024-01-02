use crate::D50;

use super::XYZ;

#[repr(C)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Lab {
    pub fn into(self, whitepoint: XYZ) -> XYZ {
        let y = (self.l + 16.0) / 116.0;
        let x = y + 0.002 * self.a;
        let z = y - 0.005 * self.b;

        let x = f_1(x) * whitepoint.x;
        let y = f_1(y) * whitepoint.y;
        let z = f_1(z) * whitepoint.z;

        XYZ { x, y, z }
    }
}

impl From<XYZ> for Lab {
    fn from(value: XYZ) -> Self {
        value.into(D50)
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
