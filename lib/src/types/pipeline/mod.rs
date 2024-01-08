use std::any::Any;

use crate::{state::Context, DupFn};

use super::Stage;

pub(crate) type PipelineEvalU16 = fn(r#in: &[u16], out: &mut [u16], data: &dyn Any);
pub(crate) type PipelineEvalF32 = fn(r#in: &[f32], out: &mut [f32], data: &dyn Any);

pub struct Pipeline {
    pub(crate) elements: Vec<Stage>,
    pub(crate) in_chans: usize,
    pub(crate) out_chans: usize,
    pub(crate) data: Option<Box<dyn Any>>,
    pub(crate) eval_u16: PipelineEvalU16,
    pub(crate) eval_f32: PipelineEvalF32,
    pub(crate) dup: DupFn,
    pub(crate) context_id: Context,
    pub(crate) save_as_8_bits: bool,
}
