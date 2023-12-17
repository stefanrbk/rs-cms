use std::any::Any;

use crate::state::Context;

use super::Signature;

pub type StageEvalFn = fn(r#in: &[f32], out: &mut [f32], stage: &Stage);
pub type StageDupFn = fn(stage: &Stage) -> Stage;

pub struct Stage {
    context_id: Context,
    r#type: Signature,
    implements: Signature,
    in_chans: usize,
    out_chans: usize,
    eval: StageEvalFn,
    dup: StageDupFn,
    data: Box<dyn Any>
}
