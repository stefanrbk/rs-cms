use super::XYZ;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XYY {
    pub x: f64,
    pub y: f64,
    pub y2: f64,
}

impl XYY {
    pub fn as_xyz(self) -> XYZ {
        let x = (self.x / self.y) * self.y2;
        let y = self.y2;
        let z = ((1f64 - self.x - self.y) / self.y) * self.y2;

        XYZ { x, y, z }
    }
}
