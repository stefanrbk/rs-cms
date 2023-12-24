use std::any::Any;

use crate::{state::Context, types::stage::curve::StageCurve};

use super::{Signature, curve::Curve};

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
    data: Box<dyn Any>,
}

mod curve;

impl Stage {
    pub(crate) fn get_curve_set(&self) -> Option<&[Curve]> {
        if let Some(data) = self.data.downcast_ref::<StageCurve>() {
            Some(&data.curves)
        } else {
            None
        }
    }

    // fn eval_curves(&self, r#in: &[f32], out: &mut [f32]) {
    //     if let Some(data) = self.get_curve_set() {
    //         let n_curves = data.len();
    //         for i in 0..n_curves {
    //             out[i] = data[i].
    //         }
    //     }
    // }
}
