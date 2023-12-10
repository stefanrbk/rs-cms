use log::Level;

use crate::{
    fixed_rest_to_int, fixed_to_int, quick_floor, round_fixed_to_int,
    state::{Context, ErrorCode},
    to_fixed_domain, Result, S15Fixed16Number, MAX_INPUT_DIMENSIONS, MAX_STAGE_CHANNELS,
};

use paste::paste;

#[derive(Clone)]
pub struct InterpParams<'table, T>
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
    pub table: &'table [T],
    pub interpolation: InterpFunction,
}

impl<'table, T: Copy> InterpParams<'table, T> {
    pub fn compute_ex(
        context_id: &Context,
        n_samples: &[usize],
        input_chan: usize,
        output_chan: usize,
        table: &'table [T],
        flags: u32,
    ) -> Result<Self> {
        // Check for maximum inputs
        if input_chan > MAX_INPUT_DIMENSIONS {
            context_id.signal_error(
                Level::Error,
                ErrorCode::Range,
                &format!(
                    "Too many input channels ({} channels, max={})",
                    input_chan, MAX_INPUT_DIMENSIONS
                ),
            );
            return Err("Invalid number of inputs");
        }

        let mut p_n_samples = [0usize; MAX_INPUT_DIMENSIONS];
        let mut p_domain = [0usize; MAX_INPUT_DIMENSIONS];

        for i in 0..input_chan {
            p_n_samples[i] = n_samples[i];
            p_domain[i] = n_samples[i] - 1;
        }

        let mut p_opta = [0usize; MAX_INPUT_DIMENSIONS];

        p_opta[0] = output_chan;
        for i in 1..input_chan {
            p_opta[i] = p_opta[i - 1] * n_samples[input_chan - i]
        }

        Ok(InterpParams {
            context_id: context_id.clone(),
            flags,
            n_inputs: input_chan,
            n_outputs: output_chan,
            n_samples: p_n_samples,
            domain: p_domain,
            opta: p_opta,
            table,
            interpolation: (context_id.get_interp_factory())(input_chan, output_chan, flags)?,
        })
    }
    pub fn compute(
        context_id: &Context,
        n_samples: usize,
        input_chan: usize,
        output_chan: usize,
        table: &'table [T],
        flags: u32,
    ) -> Result<Self> {
        let n_samples = [n_samples; MAX_INPUT_DIMENSIONS];
        Self::compute_ex(
            context_id,
            &n_samples,
            input_chan,
            output_chan,
            table,
            flags,
        )
    }
}

pub type InterpFn<T> = for<'table, 'a> fn(
    Input: &'a [T],
    Output: &'a mut [T],
    p: &'a InterpParams<'table, T>,
) -> &'a [T];

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
