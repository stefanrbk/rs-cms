use std::any::Any;

use crate::{sig, state::Context, types::stage::curve::StageCurve, Result};

use super::{curve::Curve, Signature};

pub type StageEvalFn = fn(stage: &Stage, r#in: &[f32], out: &mut [f32]);
pub type StageDupFn = fn(stage: &Stage) -> Result<Box<dyn Any>>;

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

    fn eval_curves(&self, r#in: &[f32], out: &mut [f32]) {
        if let Some(data) = self.get_curve_set() {
            let n_curves = data.len();
            for i in 0..n_curves {
                out[i] = data[i].eval_f32(r#in[i]);
            }
        }
    }

    fn dup_curve_set(&self) -> Result<Box<dyn Any>> {
        let data = self
            .data
            .downcast_ref::<StageCurve>()
            .ok_or("Stage is not a Curve")?;

        Ok(Box::new(StageCurve {
            curves: data
                .curves
                .iter()
                .map(|x| x.dup().unwrap())
                .collect::<Vec<Curve>>()
                .into(),
            context_id: data.context_id.clone(),
        }))
    }

    pub fn new_curves(context_id: &Context, curves: &[Curve]) -> Result<Self> {
        let num_chans = curves.len();
        let new_elem = StageCurve {
            curves: curves.into(),
            context_id: context_id.clone(),
        };

        Ok(Self {
            context_id: context_id.clone(),
            r#type: sig::mpe_stage::CURVE_SET,
            implements: sig::mpe_stage::CURVE_SET,
            in_chans: num_chans,
            out_chans: num_chans,
            eval: Self::eval_curves,
            dup: Self::dup_curve_set,
            data: Box::new(new_elem),
        })
    }

    pub fn new_identity_curves(context_id: &Context, num_chans: usize) -> Result<Self> {
        let curves = vec![Curve::build_gamma(context_id, 1f64).unwrap(); num_chans];
        let new_elem = StageCurve {
            curves: curves.into_boxed_slice(),
            context_id: context_id.clone(),
        };

        Ok(Self {
            context_id: context_id.clone(),
            r#type: sig::mpe_stage::CURVE_SET,
            implements: sig::mpe_stage::IDENTITY,
            in_chans: num_chans,
            out_chans: num_chans,
            eval: Self::eval_curves,
            dup: Self::dup_curve_set,
            data: Box::new(new_elem),
        })
    }
}
