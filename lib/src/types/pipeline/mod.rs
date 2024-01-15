use std::any::Any;

use crate::{
    from_f32_to_u16, from_u16_to_f32,
    state::Context,
    types::{MAT3, VEC3},
    DupFn, Result, MAX_CHANNELS, MAX_STAGE_CHANNELS,
};

use super::Stage;

pub(crate) type PipelineEvalU16 = fn(r#in: &[u16], out: &mut [u16], data: &dyn Any) -> Result<()>;
pub(crate) type PipelineEvalF32 = fn(r#in: &[f32], out: &mut [f32], data: &dyn Any) -> Result<()>;

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

impl Pipeline {
    fn bless(&mut self) -> Result<()> {
        let len = self.elements.len();

        // We can set the input/output channels only if we have elements
        if len != 0 {
            let first = &self.elements[0];
            let last = self.elements.last().unwrap();

            self.in_chans = first.in_chans;
            self.out_chans = last.out_chans;

            // Check chain consistency
            let mut prev = first;
            for i in 0..len {
                let next = &self.elements[i];

                if next.in_chans != prev.out_chans {
                    return err!(str => "Chain inconsistent");
                }

                prev = next;
            }
        }
        Ok(())
    }

    pub fn new(context_id: &Context, in_chans: usize, out_chans: usize) -> Result<Self> {
        // A value of zero in channels is allowed as placeholder
        if in_chans >= MAX_CHANNELS || out_chans >= MAX_CHANNELS {
            return err!(str => "Invalid number of channels");
        }

        let mut new_lut = Pipeline {
            elements: Vec::new(),
            in_chans,
            out_chans,
            data: None,
            eval_u16: lut_eval_u16,
            eval_f32: lut_eval_f32,
            dup: |_, _| err!(str => "Null dup"),
            context_id: context_id.clone(),
            save_as_8_bits: false,
        };

        new_lut.bless()?;

        Ok(new_lut)
    }

    pub fn context_id(&self) -> &Context {
        &self.context_id
    }

    pub fn input_channels(&self) -> usize {
        self.in_chans
    }

    pub fn output_channels(&self) -> usize {
        self.out_chans
    }

    pub fn eval_u16(&self, r#in: &[u16], out: &mut [u16]) -> Result<()> {
        (self.eval_u16)(
            r#in,
            out,
            if self.data.is_some() {
                &self.data
            } else {
                self
            },
        )
    }

    pub fn eval_f32(&self, r#in: &[f32], out: &mut [f32]) -> Result<()> {
        (self.eval_f32)(
            r#in,
            out,
            if self.data.is_some() {
                &self.data
            } else {
                self
            },
        )
    }

    pub fn dup(&self) -> Result<Self> {
        let mut new_lut = Self::new(&self.context_id, self.in_chans, self.out_chans)?;

        for mpe in &self.elements {
            let new_mpe = mpe.dup()?;

            new_lut.elements.push(new_mpe);
        }

        if self.data.is_some() {
            new_lut.data = Some((self.dup)(&self.context_id, &self.data)?);
        }

        new_lut.eval_u16 = self.eval_u16;
        new_lut.eval_f32 = self.eval_f32;
        new_lut.dup = self.dup;
        new_lut.save_as_8_bits = self.save_as_8_bits;

        new_lut.bless()?;

        Ok(new_lut)
    }

    pub fn insert_first(&mut self, mpe: Stage) -> Result<()> {
        self.elements.insert(0, mpe);

        self.bless()
    }

    pub fn push(&mut self, mpe: Stage) -> Result<()> {
        self.elements.push(mpe);

        self.bless()
    }

    pub fn remove_first(&mut self) -> Stage {
        let result = self.elements.remove(0);

        _ = self.bless();

        result
    }

    pub fn pop(&mut self) -> Option<Stage> {
        let result = self.elements.pop();

        _ = self.bless();

        result
    }

    pub fn cat(&mut self, l2: &Self) -> Result<()> {
        // If both LUTs do not have elements, we need to inherit
        // the number of channels
        if self.elements.is_empty() && l2.elements.is_empty() {
            self.in_chans = l2.in_chans;
            self.out_chans = l2.out_chans;
        }

        // Cat second
        for mpe in &l2.elements {
            // We have to dup each element
            self.push(mpe.dup()?)?;
        }

        self.bless()
    }

    pub fn set_save_as_8_bit_flag(&mut self, on: bool) -> bool {
        let prev = self.save_as_8_bits;

        self.save_as_8_bits = on;
        prev
    }

    pub fn get_elements(&self) -> &[Stage] {
        &self.elements
    }

    pub fn get_elements_mut(&mut self) -> &mut [Stage] {
        self.elements.as_mut_slice()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub(crate) fn set_optimization_parameters(
        &mut self,
        eval_16: PipelineEvalU16,
        data: Option<Box<dyn Any>>,
        dup: DupFn,
    ) {
        self.eval_u16 = eval_16;
        self.data = data;
        self.dup = dup;
    }

    pub fn eval_reverse_f32(&self, target: &[f32], result: &mut [f32], hint: &[f32]) -> Result<()> {
        let mut last_error = 1e20f64;
        let mut fx = [0f32; 4];
        let mut x = [0f32; 4];

        // Only 3->3 and 4->3 are supported
        if self.in_chans != 3 && self.in_chans != 4 {
            return err!(str => "Only 3 or 4 channel inputs are supported");
        }

        if self.out_chans != 3 {
            return err!(str => "Only 3 channel outputs are supported");
        }

        // Take the hint as starting point if specified
        x[0..3].copy_from_slice(if hint.len() >= 3 {
            // Begin at any point, we choose 1/3 of CMY axis
            &[0.3; 3]
        } else {
            // Only copy 3 channels from hint...
            &hint[0..3]
        });

        // If lut is 4-dimensions, then grab target[3], which is fixed
        x[3] = if self.in_chans == 4 { target[3] } else { 0.0 };

        // Iterate
        for _ in 0..INVERSION_MAX_ITERATIONS {
            // Get beginning fx
            self.eval_f32(&x, &mut fx)?;

            // Compute error
            let error = euclidean_distance(&x[0..3], &target[0..3]) as f64;

            // If not convergent, return last safe value
            if error >= last_error {
                break;
            }

            // Keep latest values
            last_error = error;
            result[0..self.in_chans].copy_from_slice(&x[0..self.in_chans]);

            // Found an exact match?
            if error <= 0.0 {
                break;
            }

            // Obtain slope (the Jacobian)
            let mut jacobian = MAT3::default();
            for j in 0..3 {
                let mut xd = x.clone();
                let mut fxd = [0f32; 4];

                inc_delta(&mut xd[j]);

                self.eval_f32(&xd, &mut fxd)?;

                jacobian[0][j] = (fxd[0] as f64 - fx[0] as f64) / JACOBIAN_EPSILON as f64;
                jacobian[1][j] = (fxd[1] as f64 - fx[1] as f64) / JACOBIAN_EPSILON as f64;
                jacobian[2][j] = (fxd[2] as f64 - fx[2] as f64) / JACOBIAN_EPSILON as f64;
            }

            // Solve system
            let tmp2 = VEC3 {
                x: fx[0] as f64 - target[0] as f64,
                y: fx[1] as f64 - target[1] as f64,
                z: fx[2] as f64 - target[2] as f64,
            };

            let tmp = jacobian.solve(tmp2)?;

            // Move our guess
            x[0] -= tmp[0] as f32;
            x[1] -= tmp[1] as f32;
            x[2] -= tmp[2] as f32;

            // Some clipping...
            for j in 0..3 {
                x[j] = x[j].clamp(0.0, 1.0)
            }
        }

        Ok(())
    }
}

fn lut_eval_u16(r#in: &[u16], out: &mut [u16], d: &dyn Any) -> Result<()> {
    let lut = d
        .downcast_ref::<Pipeline>()
        .ok_or("Invalid data provided to `lut_eval_u16`")?;
    let mut store0 = [0f32; MAX_STAGE_CHANNELS];
    let mut store1 = [0f32; MAX_STAGE_CHANNELS];
    let mut phase = 0usize;

    from_u16_to_f32(&r#in[..lut.in_chans], &mut store0[..lut.in_chans]);

    for mpe in &lut.elements {
        if phase == 0 {
            (mpe.eval)(&mpe, &store0, &mut store1)
        } else {
            (mpe.eval)(&mpe, &store1, &mut store0)
        };
        phase = phase ^ 1;
    }

    from_f32_to_u16(
        if phase == 0 {
            &store0[..lut.out_chans]
        } else {
            &store1[..lut.out_chans]
        },
        &mut out[..lut.out_chans],
    );

    Ok(())
}

fn lut_eval_f32(r#in: &[f32], out: &mut [f32], d: &dyn Any) -> Result<()> {
    let lut = d
        .downcast_ref::<Pipeline>()
        .ok_or("Invalid data provided to `lut_eval_f32`")?;
    let mut store0 = [0f32; MAX_STAGE_CHANNELS];
    let mut store1 = [0f32; MAX_STAGE_CHANNELS];
    let mut phase = 0usize;

    store0[..lut.in_chans].copy_from_slice(&r#in[..lut.in_chans]);

    for mpe in &lut.elements {
        if phase == 0 {
            (mpe.eval)(&mpe, &store0, &mut store1)
        } else {
            (mpe.eval)(&mpe, &store1, &mut store0)
        };
        phase = phase ^ 1;
    }

    out[..lut.out_chans].copy_from_slice(if phase == 0 {
        &store0[..lut.out_chans]
    } else {
        &store1[..lut.out_chans]
    });

    Ok(())
}

const JACOBIAN_EPSILON: f32 = 1e-3f32;
const INVERSION_MAX_ITERATIONS: usize = 30;

fn inc_delta(val: &mut f32) {
    if *val < (1.0 - JACOBIAN_EPSILON) {
        *val += JACOBIAN_EPSILON;
    } else {
        *val -= JACOBIAN_EPSILON;
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0f32;

    let len = a.len().min(b.len());

    for i in 0..len {
        let dif = b[i] - a[i];
        sum += dif * dif;
    }

    sum.sqrt()
}
