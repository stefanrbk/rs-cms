use super::XYZ;

#[repr(C)]
pub struct XYY {
    pub x: f64,
    pub y: f64,
    pub y2: f64,
}

impl From<XYZ> for XYY {
    fn from(value: XYZ) -> Self {
        let i_sum = 1f64 / (value.x + value.y + value.z);

        let x = value.x * i_sum;
        let y = value.y * i_sum;
        let y2 = value.y;

        XYY { x, y, y2 }
    }
}
