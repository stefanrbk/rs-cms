use crate::types::InterpParams;

pub struct StageCLut<T: Copy> {
    pub tab: Box<[T]>,
    pub params: InterpParams<T>,
}
