use std::ops::{Index, IndexMut};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VEC3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
