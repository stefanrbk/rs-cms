use std::any::Any;

use crate::{quick_saturate_word, sig, state::Context, types::stage::curve::StageCurve, Result};

use super::{curve::Curve, Signature};

pub type StageEvalFn = fn(stage: &Stage, r#in: &[f32], out: &mut [f32]);
pub type StageDupFn = fn(stage: &Stage) -> Result<Box<dyn Any>>;

pub struct Stage {
    pub(crate) context_id: Context,
    pub(crate) r#type: Signature,
    pub(crate) implements: Signature,
    pub(crate) in_chans: usize,
    pub(crate) out_chans: usize,
    pub(crate) eval: StageEvalFn,
    pub(crate) dup: StageDupFn,
    pub(crate) data: Box<dyn Any>,
}

mod curve;

impl Stage {
    fn new(
        context_id: &Context,
        r#type: Signature,
        in_chans: usize,
        out_chans: usize,
        eval: StageEvalFn,
        dup: StageDupFn,
        data: Box<dyn Any>,
    ) -> Self {
        Self {
            context_id: context_id.clone(),
            r#type,
            implements: r#type,
            in_chans,
            out_chans,
            eval,
            dup,
            data,
        }
    }

    fn eval_identity(&self, r#in: &[f32], out: &mut [f32]) {
        out[..self.in_chans].copy_from_slice(&r#in[..self.in_chans])
    }

    pub fn new_identity(context_id: &Context, num_chans: usize) -> Result<Self> {
        Ok(Self::new(
            &context_id,
            sig::mpe_stage::IDENTITY,
            num_chans,
            num_chans,
            Self::eval_curves,
            Self::dup_curve_set,
            Box::new(0),
        ))
    }

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

        let mut new_lut = Self::new(
            &context_id,
            sig::mpe_stage::CURVE_SET,
            num_chans,
            num_chans,
            Self::eval_curves,
            Self::dup_curve_set,
            Box::new(new_elem),
        );

        new_lut.implements = sig::mpe_stage::IDENTITY;

        Ok(new_lut)
    }
}

fn from_f32_to_u16(r#in: &[f32], out: &mut [u16]) {
    let len = r#in.len();
    for i in 0..len {
        out[i] = quick_saturate_word(i as f64 * 65535f64);
    }
}

fn from_u16_to_f32(r#in: &[u16], out: &mut [f32]) {
    let len = r#in.len();
    for i in 0..len {
        out[i] = i as f32 / 65535f32;
    }
}
