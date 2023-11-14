use crate::{state::Context, MAX_INPUT_DIMENSIONS};

#[derive(Clone)]
pub struct InterpParams<T>
where
    T: Copy,
{
    pub context_id: Context,
    pub flags: u32,
    pub n_inputs: usize,
    pub n_outputs: usize,
    pub n_samples: [usize; MAX_INPUT_DIMENSIONS],
    pub domain: [usize; MAX_INPUT_DIMENSIONS],
    pub opta: [usize; MAX_INPUT_DIMENSIONS],
    pub table: Box<[T]>,
    pub interpolation: InterpFunction,
}

pub type InterpFn<T> = for<'a> fn(Input: &'a [T], Output: &'a mut [T], p: &'a InterpParams<T>) -> &'a [T];

#[derive(Clone)]
pub enum InterpFunction {
    F32(InterpFn<f32>),
    U16(InterpFn<u16>),
}

impl InterpFunction {
    pub const fn is_f32(&self) -> bool {
        matches!(*self, Self::F32(_))
    }
    pub const fn is_u16(&self) -> bool {
        matches!(*self, Self::U16(_))
    }
    pub fn is_f32_and(self, f: impl FnOnce(InterpFn<f32>) -> bool) -> bool {
        match self {
            Self::U16(_) => false,
            Self::F32(x) => f(x),
        }
    }
    pub fn is_u16_and(self, f: impl FnOnce(InterpFn<u16>) -> bool) -> bool {
        match self {
            Self::U16(x) => f(x),
            Self::F32(_) => false,
        }
    }
}
