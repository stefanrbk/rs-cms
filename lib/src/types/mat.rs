use std::ops::{Index, IndexMut, Mul};

use crate::{Result, MATRIX_DET_TOLERANCE};

use super::VEC3;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MAT3 {
    pub x: VEC3,
    pub y: VEC3,
    pub z: VEC3,
}

impl MAT3 {
    pub const IDENTITY: Self = MAT3 {
        x: VEC3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        y: VEC3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        z: VEC3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    pub fn is_identity(self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !close_enough(self[i][j], Self::IDENTITY[i][j]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn inverse(self) -> Result<Self> {
        let c0 = self[1][1] * self[2][2] - self[1][2] * self[2][1];
        let c1 = -self[1][0] * self[2][2] + self[1][2] * self[2][0];
        let c2 = self[1][0] * self[2][1] - self[1][1] * self[2][0];

        let det = self[0][0] * c0 + self[0][1] * c1 + self[0][2] * c2;

        if det.abs() < MATRIX_DET_TOLERANCE {
            return err!(str => "Singular matrix; can't invert");
        }

        Ok(Self {
            x: VEC3 {
                x: c0 / det,
                y: (self[0][2] * self[2][1] - self[0][1] * self[2][2]) / det,
                z: (self[0][1] * self[1][2] - self[0][2] * self[1][1]) / det,
            },
            y: VEC3 {
                x: c1 / det,
                y: (self[0][0] * self[2][2] - self[0][2] * self[2][0]) / det,
                z: (self[0][2] * self[1][0] - self[0][0] * self[1][2]) / det,
            },
            z: VEC3 {
                x: c2 / det,
                y: (self[0][1] * self[2][0] - self[0][0] * self[2][1]) / det,
                z: (self[0][0] * self[1][1] - self[0][1] * self[1][0]) / det,
            },
        })
    }

    pub fn solve(self, b: VEC3) -> Result<VEC3> {
        let a = self.inverse()?;

        Ok(a * b)
    }
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

impl Mul for MAT3 {
    type Output = MAT3;

    fn mul(self, rhs: Self) -> Self::Output {
        macro_rules! row_col {
            ($i:expr, $j:expr) => {
                self[$i][0] * rhs[0][$j] + self[$i][1] * rhs[1][$j] + self[$i][2] * rhs[2][$j]
            };
        }

        Self::Output {
            x: VEC3 {
                x: row_col!(0, 0),
                y: row_col!(0, 1),
                z: row_col!(0, 2),
            },
            y: VEC3 {
                x: row_col!(1, 0),
                y: row_col!(1, 1),
                z: row_col!(1, 2),
            },
            z: VEC3 {
                x: row_col!(2, 0),
                y: row_col!(2, 1),
                z: row_col!(2, 2),
            },
        }
    }
}

impl Mul<VEC3> for MAT3 {
    type Output = VEC3;

    fn mul(self, rhs: VEC3) -> Self::Output {
        VEC3 {
            x: self[0].x * rhs.x + self[0].y * rhs.y + self[0].z * rhs.z,
            y: self[1].x * rhs.x + self[1].y * rhs.y + self[1].z * rhs.z,
            z: self[2].x * rhs.x + self[2].y * rhs.y + self[2].z * rhs.z,
        }
    }
}

fn close_enough(a: f64, b: f64) -> bool {
    (b - a).abs() < (1.0 / 65535.0)
}
