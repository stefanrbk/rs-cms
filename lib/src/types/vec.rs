use std::ops::{Index, IndexMut, Sub, Mul};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VEC3 {
    pub fn dot(self, v: Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn len_sqr(self) -> f64 {
        self.dot(self)
    }

    pub fn len(self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn distance(self, b: Self) -> f64 {
        (self - b).len()
    }
}

impl Index<usize> for VEC3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y, &self.z][index]
    }
}

impl IndexMut<usize> for VEC3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        [&mut self.x, &mut self.y, &mut self.z][index]
    }
}

impl Sub for VEC3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for VEC3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
}
