use std::ops::{Index, IndexMut};

use super::VEC3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAT3 {
    pub x: VEC3,
    pub y: VEC3,
    pub z: VEC3,
}

impl Index<usize> for MAT3 {
    type Output = VEC3;

    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y, &self.z][index]
    }
}

impl IndexMut<usize> for MAT3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        [&mut self.x, &mut self.y, &mut self.z][index]
    }
}
