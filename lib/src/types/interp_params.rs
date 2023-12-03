use log::Level;

use crate::{
    fixed_rest_to_int, fixed_to_int, quick_floor, round_fixed_to_int,
    state::{Context, ErrorCode},
    to_fixed_domain, Result, S15Fixed16Number, MAX_INPUT_DIMENSIONS,
};

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

impl<T: Copy> InterpParams<T> {
    pub fn compute_ex(
        context_id: &Context,
        n_samples: &[usize],
        input_chan: usize,
        output_chan: usize,
        table: Box<[T]>,
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
        table: Box<[T]>,
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

pub type InterpFn<T> =
    for<'a> fn(Input: &'a [T], Output: &'a mut [T], p: &'a InterpParams<T>) -> &'a [T];

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

#[inline]
fn linear_interp_u16(a: S15Fixed16Number, l: S15Fixed16Number, h: S15Fixed16Number) -> u16 {
    let dif = (h - l) as u32 * a as u32 + 0x8000;
    let dif = (dif >> 16) + l as u32;
    dif as u16
}

fn lin_lerp_1d_u16(value: &[u16], output: &mut [u16], p: &InterpParams<u16>) {
    let lut_table = &p.table;

    // if last value or just one point
    if value[0] == 0xffff || p.domain[0] == 0 {
        output[0] = lut_table[p.domain[0]];
    } else {
        let val3 = p.domain[0] as i32 * value[0] as i32;
        let val3 = to_fixed_domain(val3);

        let cell0 = fixed_to_int(val3);
        let rest = fixed_rest_to_int(val3);

        let y0 = lut_table[cell0 as usize];
        let y1 = lut_table[cell0 as usize + 1];

        output[0] = linear_interp_u16(rest, y0 as i32, y1 as i32);
    }
}

#[inline]
fn fclamp(v: f32) -> f32 {
    if (v < 1.0e-9f32) || v.is_nan() {
        0f32
    } else {
        if v > 1f32 {
            1f32
        } else {
            v
        }
    }
}

#[inline]
fn linear_interp_f32(a: f32, l: f32, h: f32) -> f32 {
    l + (h - l) * a
}

fn lin_lerp_1d_f32(value: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let lut_table = &p.table;

    let val2 = fclamp(value[0]);

    // if last value...
    if val2 == 1f32 || p.domain[0] == 0 {
        output[0] = lut_table[p.domain[0]];
    } else {
        let val2 = p.domain[0] as f32 * val2;

        let cell0 = val2.floor() as i32;
        let cell1 = val2.ceil() as i32;

        // Rest is 16 LSB bits
        let rest = val2 - cell0 as f32;

        let y0 = lut_table[cell0 as usize];
        let y1 = lut_table[cell1 as usize];

        output[0] = linear_interp_f32(rest, y0, y1);
    }
}

fn eval_1_input_u16(input: &[u16], output: &mut [u16], p16: &InterpParams<u16>) {
    let lut_table = &p16.table;

    // if last value...
    if input[0] == 0xffff || p16.domain[0] == 0 {
        let y0 = p16.domain[0] * p16.opta[0];

        for out_chan in 0..p16.n_outputs {
            output[out_chan] = lut_table[y0 + out_chan];
        }
    } else {
        let v = input[0] as i32 * p16.domain[0] as i32;
        let fk = to_fixed_domain(v);

        let k0 = fixed_to_int(fk);
        let rk = fixed_rest_to_int(fk) as u16;

        let k1 = k0 + if input[0] != 0xffff { 1 } else { 0 };

        let k0 = p16.opta[0] as i32 * k0;
        let k1 = p16.opta[0] as i32 * k1;

        for out_chan in 0..p16.n_outputs {
            output[out_chan] = linear_interp_u16(
                rk as i32,
                lut_table[k0 as usize + out_chan] as i32,
                lut_table[k1 as usize + out_chan] as i32,
            );
        }
    }
}
fn eval_1_input_f32(value: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let lut_table = &p.table;

    let val2 = fclamp(value[0]);

    // if last value...
    if val2 == 1f32 || p.domain[0] == 0 {
        let start = p.domain[0] * p.opta[0];

        for out_chan in 0..p.n_outputs {
            output[out_chan] = lut_table[start + out_chan];
        }
    } else {
        let val2 = p.domain[0] as f32 * val2;

        let cell0 = val2.floor() as i32;
        let cell1 = val2.ceil() as i32;

        // Rest is 16 LSB bits
        let rest = val2 - cell0 as f32;

        let cell0 = cell0 * p.opta[0] as i32;
        let cell1 = cell1 * p.opta[0] as i32;

        for out_chan in 0..p.n_outputs {
            let y0 = lut_table[cell0 as usize + out_chan];
            let y1 = lut_table[cell1 as usize + out_chan];

            output[0] = linear_interp_f32(rest, y0, y1);
        }
    }
}

fn bilinear_interp_u16(input: &[u16], output: &mut [u16], p: &InterpParams<u16>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let fx = to_fixed_domain(p.domain[0] as i32 * input[0] as i32);
    let x0 = fixed_to_int(fx);
    let rx = fixed_rest_to_int(fx);

    let fy = to_fixed_domain(p.domain[1] as i32 * input[1] as i32);
    let y0 = fixed_to_int(fy);
    let ry = fixed_rest_to_int(fy);

    let x0 = p.opta[1] as i32 * x0;
    let x1 = x0
        + (if input[0] == 0xffff {
            0
        } else {
            p.opta[1] as i32
        });

    let y0 = p.opta[0] as i32 * y0;
    let y1 = y0
        + (if input[1] == 0xffff {
            0
        } else {
            p.opta[0] as i32
        });

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr) => {
                lut_table[$i as usize + $j as usize + out_chan] as i32
            };
        }
        macro_rules! lerp {
            ($a:expr, $l:expr, $h:expr) => {
                ($l + round_fixed_to_int(($h - $l) * $a))
            };
        }
        let d00 = dens!(x0, y0);
        let d01 = dens!(x0, y1);
        let d10 = dens!(x1, y0);
        let d11 = dens!(x1, y1);

        let dx0 = lerp!(rx, d00, d10);
        let dx1 = lerp!(rx, d01, d11);

        let dxy = lerp!(ry, dx0, dx1);

        output[out_chan] = dxy as u16;
    }
}

fn bilinear_interp_f32(input: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;

    let x0 = quick_floor(px as f64);
    let y0 = quick_floor(py as f64);

    let fx = px - x0 as f32;
    let fy = py - y0 as f32;

    let x0 = p.opta[1] as i32 * x0;
    let x1 = x0
        + (if input[0] >= 1f32 {
            0
        } else {
            p.opta[1] as i32
        });

    let y0 = p.opta[0] as i32 * y0;
    let y1 = y0
        + (if input[1] >= 1f32 {
            0
        } else {
            p.opta[0] as i32
        });

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr) => {
                lut_table[$i as usize + $j as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l:expr, $h:expr) => {
                ($l + ($h - $l) * $a)
            };
        }
        let d00 = dens!(x0, y0);
        let d01 = dens!(x0, y1);
        let d10 = dens!(x1, y0);
        let d11 = dens!(x1, y1);

        let dx0 = lerp!(fx, d00, d10);
        let dx1 = lerp!(fx, d01, d11);

        let dxy = lerp!(fy, dx0, dx1);

        output[out_chan] = dxy;
    }
}
