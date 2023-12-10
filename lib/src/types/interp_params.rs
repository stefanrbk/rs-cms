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

fn trilinear_interp_u16(input: &[u16], output: &mut [u16], p: &InterpParams<u16>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let fx = to_fixed_domain(p.domain[0] as i32 * input[0] as i32);
    let x0 = fixed_to_int(fx);
    let rx = fixed_rest_to_int(fx);

    let fy = to_fixed_domain(p.domain[1] as i32 * input[1] as i32);
    let y0 = fixed_to_int(fy);
    let ry = fixed_rest_to_int(fy);

    let fz = to_fixed_domain(p.domain[2] as i32 * input[2] as i32);
    let z0 = fixed_to_int(fz);
    let rz = fixed_rest_to_int(fz);

    let x0 = p.opta[2] as i32 * x0;
    let x1 = x0
        + (if input[0] == 0xffff {
            0
        } else {
            p.opta[2] as i32
        });

    let y0 = p.opta[1] as i32 * y0;
    let y1 = y0
        + (if input[1] == 0xffff {
            0
        } else {
            p.opta[1] as i32
        });

    let z0 = p.opta[0] as i32 * z0;
    let z1 = z0
        + (if input[2] == 0xffff {
            0
        } else {
            p.opta[0] as i32
        });

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan] as i32
            };
        }
        macro_rules! lerp {
            ($a:expr, $l:expr, $h:expr) => {
                ($l + round_fixed_to_int(($h - $l) * $a))
            };
        }

        let d000 = dens!(x0, y0, z0);
        let d001 = dens!(x0, y0, z1);
        let d010 = dens!(x0, y1, z0);
        let d011 = dens!(x0, y1, z1);

        let d100 = dens!(x1, y0, z0);
        let d101 = dens!(x1, y0, z1);
        let d110 = dens!(x1, y1, z0);
        let d111 = dens!(x1, y1, z1);

        let dx00 = lerp!(rx, d000, d100);
        let dx01 = lerp!(rx, d001, d101);
        let dx10 = lerp!(rx, d010, d110);
        let dx11 = lerp!(rx, d011, d111);

        let dxy0 = lerp!(ry, dx00, dx10);
        let dxy1 = lerp!(ry, dx01, dx11);

        let dxyz = lerp!(rz, dxy0, dxy1);

        output[out_chan] = dxyz as u16;
    }
}

fn trilinear_interp_f32(input: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;
    let pz = fclamp(input[2]) * p.domain[2] as f32;

    let x0 = px.floor() as i32;
    let y0 = py.floor() as i32;
    let z0 = pz.floor() as i32;

    let fx = px - x0 as f32;
    let fy = py - y0 as f32;
    let fz = pz - z0 as f32;

    let x0 = p.opta[2] as i32 * x0;
    let x1 = x0
        + (if fclamp(input[0]) >= 1f32 {
            0
        } else {
            p.opta[2] as i32
        });

    let y0 = p.opta[1] as i32 * y0;
    let y1 = y0
        + (if fclamp(input[1]) >= 1f32 {
            0
        } else {
            p.opta[1] as i32
        });

    let z0 = p.opta[0] as i32 * z0;
    let z1 = z0
        + (if fclamp(input[2]) >= 1f32 {
            0
        } else {
            p.opta[0] as i32
        });

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l:expr, $h:expr) => {
                ($l + ($h - $l) * $a)
            };
        }

        let d000 = dens!(x0, y0, z0);
        let d001 = dens!(x0, y0, z1);
        let d010 = dens!(x0, y1, z0);
        let d011 = dens!(x0, y1, z1);

        let d100 = dens!(x1, y0, z0);
        let d101 = dens!(x1, y0, z1);
        let d110 = dens!(x1, y1, z0);
        let d111 = dens!(x1, y1, z1);

        let dx00 = lerp!(fx, d000, d100);
        let dx01 = lerp!(fx, d001, d101);

        let dx10 = lerp!(fx, d010, d110);
        let dx11 = lerp!(fx, d011, d111);

        let dxy0 = lerp!(fy, dx00, dx10);
        let dxy1 = lerp!(fy, dx01, dx11);

        let dxyz = lerp!(fz, dxy0, dxy1);

        output[out_chan] = dxyz;
    }
}

fn tetrahedral_interp_u16(input: &[u16], mut output: &mut [u16], p: &InterpParams<u16>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let fx = to_fixed_domain(p.domain[0] as i32 * input[0] as i32);
    let fy = to_fixed_domain(p.domain[1] as i32 * input[1] as i32);
    let fz = to_fixed_domain(p.domain[2] as i32 * input[2] as i32);

    let x0 = fixed_to_int(fx);
    let y0 = fixed_to_int(fy);
    let z0 = fixed_to_int(fz);

    let rx = fixed_rest_to_int(fx);
    let ry = fixed_rest_to_int(fy);
    let rz = fixed_rest_to_int(fz);

    let x0 = p.opta[2] as i32 * x0;
    let mut x1 = (x0
        + (if input[0] == 0xffff {
            0
        } else {
            p.opta[2] as i32
        })) as usize;

    let y0 = p.opta[1] as i32 * y0;
    let mut y1 = (y0
        + (if input[1] == 0xffff {
            0
        } else {
            p.opta[1] as i32
        })) as usize;

    let z0 = p.opta[0] as i32 * z0;
    let mut z1 = (z0
        + (if input[2] == 0xffff {
            0
        } else {
            p.opta[0] as i32
        })) as usize;

    let mut lut_table = &lut_table[((x0 + y0 + z0) as usize)..];

    if rx >= ry {
        if ry >= rz {
            y1 += x1;
            z1 += y1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c3 = c3 - c2;
                let c2 = c2 - c1;
                let c1 = c1 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        } else if rz >= rx {
            x1 += z1;
            y1 += x1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c2 = c2 - c1;
                let c1 = c1 - c3;
                let c3 = c3 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        } else {
            z1 += x1;
            y1 += z1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c2 = c2 - c3;
                let c3 = c3 - c1;
                let c1 = c1 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        }
    } else {
        if rx >= rz {
            x1 += y1;
            z1 += x1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c3 = c3 - c1;
                let c1 = c1 - c2;
                let c2 = c2 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        } else if ry >= rz {
            z1 += y1;
            x1 += z1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c1 = c1 - c3;
                let c3 = c3 - c2;
                let c2 = c2 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        } else {
            y1 += z1;
            x1 += y1;
            for _ in 0..total_out {
                let c1 = lut_table[x1];
                let c2 = lut_table[y1];
                let c3 = lut_table[z1];
                let c0 = lut_table[0];
                lut_table = &lut_table[1..];
                let c1 = c1 - c2;
                let c2 = c2 - c3;
                let c3 = c3 - c0;
                let rest = c1 as i32 * rx + c2 as i32 * ry + c3 as i32 * rz + 0x8001;
                output[0] = (c0 as i32 + ((rest + (rest >> 16)) >> 16)) as u16;
                output = &mut output[1..];
            }
        }
    }
}
macro_rules! dens {
    ($lut_table:expr, $out_chan:expr; $i:expr, $j:expr, $k:expr) => {
        $lut_table[($i + $j + $k) as usize + $out_chan] as i32
    };
}

macro_rules! lut_u16 {
    (($lut:expr, $output:expr, $total_out:expr) =>
        {$rx:expr, $ry:expr, $rz:expr; $x0:expr, $y0:expr, $z0:expr},
        $({$($c_x_:expr, $c_y_:expr, $c_z_:expr);*}),*) => {
            for out_chan in 0..$total_out {
                let c0 = dens!($lut, out_chan; $x0, $y0, $z0);

                let (c1, c2, c3) = _lut!(($lut, out_chan, c0) => $({$($c_x_, $c_y_, $c_z_);*}),*);

                let rest = c1 * $rx + c2 * $ry + c3 * $rz;
                $output[out_chan] = (c0 + round_fixed_to_int(to_fixed_domain(rest))) as u16;
            }
    };
}

macro_rules! lut_f32 {
    (($lut:expr, $output:expr, $total_out:expr) =>
        {$rx:expr, $ry:expr, $rz:expr; $x0:expr, $y0:expr, $z0:expr},
        $({$($c_x_:expr, $c_y_:expr, $c_z_:expr);*}),*) => {
            for out_chan in 0..$total_out {
                let c0 = dens!($lut, out_chan; $x0, $y0, $z0);

                let (c1, c2, c3) = _lut!(($lut, out_chan, c0) => $({$($c_x_, $c_y_, $c_z_);*}),*);

                $output[out_chan] = c0 as f32 + c1 as f32 * $rx + c2 as f32 * $ry + c3 as f32 * $rz;
            }
    };
}

macro_rules! _lut {
    (($lut:expr, $out_chan:expr, $c0:expr) =>
        {$c1x1:expr, $c1y1:expr, $c1z1:expr},
        {$c2x1:expr, $c2y1:expr, $c2z1:expr; $c2x2:expr, $c2y2:expr, $c2z2:expr},
        {$c3x1:expr, $c3y1:expr, $c3z1:expr; $c3x2:expr, $c3y2:expr, $c3z2:expr}) => {
            (dens!($lut, $out_chan; $c1x1, $c1y1, $c1z1) - $c0,
                dens!($lut, $out_chan; $c2x1, $c2y1, $c2z1) - dens!($lut, $out_chan; $c2x2, $c2y2, $c2z2),
                dens!($lut, $out_chan; $c3x1, $c3y1, $c3z1) - dens!($lut, $out_chan; $c3x2, $c3y2, $c3z2))
        };
        (($lut:expr, $out_chan:expr, $c0:expr) =>
        {$c1x1:expr, $c1y1:expr, $c1z1:expr; $c1x2:expr, $c1y2:expr, $c1z2:expr},
        {$c2x1:expr, $c2y1:expr, $c2z1:expr},
        {$c3x1:expr, $c3y1:expr, $c3z1:expr; $c3x2:expr, $c3y2:expr, $c3z2:expr}) => {
            (dens!($lut, $out_chan; $c1x1, $c1y1, $c1z1) - dens!($lut, $out_chan; $c1x2, $c1y2, $c1z2),
                dens!($lut, $out_chan; $c2x1, $c2y1, $c2z1) - $c0,
                dens!($lut, $out_chan; $c3x1, $c3y1, $c3z1) - dens!($lut, $out_chan; $c3x2, $c3y2, $c3z2))
    };
    (($lut:expr, $out_chan:expr, $c0:expr) =>
        {$c1x1:expr, $c1y1:expr, $c1z1:expr; $c1x2:expr, $c1y2:expr, $c1z2:expr},
        {$c2x1:expr, $c2y1:expr, $c2z1:expr; $c2x2:expr, $c2y2:expr, $c2z2:expr},
        {$c3x1:expr, $c3y1:expr, $c3z1:expr}) => {
            (dens!($lut, $out_chan; $c1x1, $c1y1, $c1z1) - dens!($lut, $out_chan; $c1x2, $c1y2, $c1z2),
                dens!($lut, $out_chan; $c2x1, $c2y1, $c2z1) - dens!($lut, $out_chan; $c2x2, $c2y2, $c2z2),
                dens!($lut, $out_chan; $c3x1, $c3y1, $c3z1) - $c0)
    };
}

fn tetrahedral_interp_f32(input: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let lut_table = &p.table;

    let total_out = p.n_outputs;

    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;
    let pz = fclamp(input[2]) * p.domain[2] as f32;

    let x0 = px.floor() as i32;
    let y0 = py.floor() as i32;
    let z0 = pz.floor() as i32;

    let rx = px - x0 as f32;
    let ry = py - y0 as f32;
    let rz = pz - z0 as f32;

    let x0 = p.opta[2] as i32 * x0;
    let y0 = p.opta[1] as i32 * y0;
    let z0 = p.opta[0] as i32 * z0;

    let x1 = x0
        + (if fclamp(input[0]) >= 1f32 {
            0
        } else {
            p.opta[2] as i32
        });

    let y1 = y0
        + (if fclamp(input[1]) >= 1f32 {
            0
        } else {
            p.opta[1] as i32
        });

    let z1 = z0
        + (if fclamp(input[2]) >= 1f32 {
            0
        } else {
            p.opta[0] as i32
        });

    if rx >= ry {
        if ry >= rz
        // rx >= ry >= rz
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z0; x1, y0, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if rz >= rx
        // rz >= rx >= ry
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z1; x0, y0, z1},
                {x1, y1, z1; x1, y0, z1},
                {x0, y0, z1}
            );
        } else
        // rx >= rz >= ry
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z1; x1, y0, z1},
                {x1, y0, z1; x1, y0, z0}
            );
        }
    } else
    // ry >= rx
    {
        if rx >= rz
        // ry >= rx >= rz
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z0; x0, y1, z0},
                {x0, y1, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if ry >= rz
        // ry >= rz >= rx
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z0},
                {x0, y1, z1; x0, y1, z0}
            );
        } else
        // rz >= ry >= rx
        {
            lut_f32!(
                (lut_table, output, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z1; x0, y0, z1},
                {x0, y0, z1}
            );
        }
    }
}

fn eval_4_inputs_u16(input: &[u16], output: &mut [u16], p16: &InterpParams<u16>) {
    let mut tmp1 = [0u16; MAX_STAGE_CHANNELS];
    let mut tmp2 = [0u16; MAX_STAGE_CHANNELS];

    let total_out = p16.n_outputs;

    let fk = to_fixed_domain(p16.domain[0] as i32 * input[0] as i32);
    let fx = to_fixed_domain(p16.domain[1] as i32 * input[1] as i32);
    let fy = to_fixed_domain(p16.domain[2] as i32 * input[2] as i32);
    let fz = to_fixed_domain(p16.domain[3] as i32 * input[3] as i32);

    let k0 = fixed_to_int(fk);
    let x0 = fixed_to_int(fx);
    let y0 = fixed_to_int(fy);
    let z0 = fixed_to_int(fz);

    let rk = fixed_rest_to_int(fk);
    let rx = fixed_rest_to_int(fx);
    let ry = fixed_rest_to_int(fy);
    let rz = fixed_rest_to_int(fz);

    let k0 = p16.opta[3] as i32 * k0;
    let x0 = p16.opta[2] as i32 * x0;
    let y0 = p16.opta[1] as i32 * y0;
    let z0 = p16.opta[0] as i32 * z0;

    let k1 = k0
        + (if input[0] == 0xffff {
            0
        } else {
            p16.opta[3] as i32
        });

    let x1 = x0
        + (if input[1] == 0xffff {
            0
        } else {
            p16.opta[2] as i32
        });

    let y1 = y0
        + (if input[2] == 0xffff {
            0
        } else {
            p16.opta[1] as i32
        });

    let z1 = z0
        + (if input[3] == 0xffff {
            0
        } else {
            p16.opta[0] as i32
        });

    let lut_table = &p16.table[(k0 as usize)..];

    if rx >= ry {
        if ry >= rz
        // rx >= ry >= rz
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z0; x1, y0, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if rz >= rx
        // rz >= rx >= ry
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z1; x0, y0, z1},
                {x1, y1, z1; x1, y0, z1},
                {x0, y0, z1}
            );
        } else
        // rx >= rz >= ry
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z1; x1, y0, z1},
                {x1, y0, z1; x1, y0, z0}
            );
        }
    } else
    // ry >= rx
    {
        if rx >= rz
        // ry >= rx >= rz
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z0; x0, y1, z0},
                {x0, y1, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if ry >= rz
        // ry >= rz >= rx
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z0},
                {x0, y1, z1; x0, y1, z0}
            );
        } else
        // rz >= ry >= rx
        {
            lut_u16!(
                (lut_table, tmp1, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z1; x0, y0, z1},
                {x0, y0, z1}
            );
        }
    }

    let lut_table = &p16.table[(k1 as usize)..];

    if rx >= ry {
        if ry >= rz
        // rx >= ry >= rz
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z0; x1, y0, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if rz >= rx
        // rz >= rx >= ry
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z1; x0, y0, z1},
                {x1, y1, z1; x1, y0, z1},
                {x0, y0, z1}
            );
        } else
        // rx >= rz >= ry
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y0, z0},
                {x1, y1, z1; x1, y0, z1},
                {x1, y0, z1; x1, y0, z0}
            );
        }
    } else
    // ry >= rx
    {
        if rx >= rz
        // ry >= rx >= rz
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z0; x0, y1, z0},
                {x0, y1, z0},
                {x1, y1, z1; x1, y1, z0}
            );
        } else if ry >= rz
        // ry >= rz >= rx
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z0},
                {x0, y1, z1; x0, y1, z0}
            );
        } else
        // rz >= ry >= rx
        {
            lut_u16!(
                (lut_table, tmp2, total_out) =>
                {rx, ry, rz; x0, y0, z0},
                {x1, y1, z1; x0, y1, z1},
                {x0, y1, z1; x0, y0, z1},
                {x0, y0, z1}
            );
        }
    }

    for i in 0..p16.n_outputs {
        output[i] = linear_interp_u16(rk, tmp1[i] as i32, tmp2[i] as i32);
    }
}

fn eval_4_inputs_f32(input: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
    let mut tmp1 = [0f32; MAX_STAGE_CHANNELS];
    let mut tmp2 = [0f32; MAX_STAGE_CHANNELS];

    let pk = fclamp(input[0]) * p.domain[0] as f32;
    let k0 = quick_floor(pk as f64) as i32;
    let rest = pk - k0 as f32;

    let k0 = p.opta[3] as i32 * k0;
    let k1 = k0
        + (if fclamp(input[0]) >= 1f32 {
            0
        } else {
            p.opta[3] as i32
        });

    let mut p1 = p.clone();
    p1.domain.copy_within(1..4, 0);

    let t = &p.table[(k0 as usize)..];
    p1.table = t;

    tetrahedral_interp_f32(&input[1..], &mut tmp1, &p1);

    let t = &p.table[(k1 as usize)..];
    p1.table = t;

    tetrahedral_interp_f32(&input[1..], &mut tmp2, &p1);

    for i in 0..p.n_outputs {
        let y0 = tmp1[i];
        let y1 = tmp2[i];

        output[i] = y0 + (y1 - y0) * rest;
    }
}

macro_rules! eval_fns {
    ($n:expr, $nm:expr) => {
        paste!{
            fn [<eval_ $n _inputs_u16>](input: &[u16], output: &mut [u16], p16: &InterpParams<u16>) {
                let mut tmp1 = [0u16; MAX_STAGE_CHANNELS];
                let mut tmp2 = [0u16; MAX_STAGE_CHANNELS];

                let fk = to_fixed_domain(p16.domain[0] as i32 * input[0] as i32);
                let k0 = fixed_to_int(fk);
                let rk = fixed_rest_to_int(fk);

                let k0 = p16.opta[$nm] as i32 * k0;
                let k1 = p16.opta[$nm] as i32 * (k0
                    + (if input[0] == 0xffff {
                        0
                    } else {
                        1
                    }));

                let mut p1 = p16.clone();
                p1.domain.copy_within(1..$n, 0);

                let t = &p16.table[(k0 as usize)..];
                p1.table = t;

                [<eval_ $nm _inputs_u16>](&input[1..], &mut tmp1, &p1);

                let t = &p16.table[(k1 as usize)..];
                p1.table = t;

                [<eval_ $nm _inputs_u16>](&input[1..], &mut tmp2, &p1);

                for i in 0..p16.n_outputs {
                    output[i] = linear_interp_u16(rk, tmp1[i] as i32, tmp2[i] as i32);
                }
            }

            fn [<eval_ $n _inputs_f32>](input: &[f32], output: &mut [f32], p: &InterpParams<f32>) {
                let mut tmp1 = [0f32; MAX_STAGE_CHANNELS];
                let mut tmp2 = [0f32; MAX_STAGE_CHANNELS];

                let pk = fclamp(input[0]) * p.domain[0] as f32;
                let k0 = quick_floor(pk as f64) as i32;
                let rest = pk - k0 as f32;

                let k0 = p.opta[$nm] as i32 * k0;
                let k1 = p.opta[3] as i32 * (k0
                    + (if fclamp(input[0]) >= 1f32 {
                        0
                    } else {
                        1
                    }));

                let mut p1 = p.clone();
                p1.domain.copy_within(1..$n, 0);

                let t = &p.table[(k0 as usize)..];
                p1.table = t;

                [<eval_ $nm _inputs_f32>](&input[1..], &mut tmp1, &p1);

                let t = &p.table[(k1 as usize)..];
                p1.table = t;

                [<eval_ $nm _inputs_f32>](&input[1..], &mut tmp2, &p1);

                for i in 0..p.n_outputs {
                    let y0 = tmp1[i];
                    let y1 = tmp2[i];

                    output[i] = y0 + (y1 - y0) * rest;
                }
            }
        }
    };
}
eval_fns!(5, 4);
eval_fns!(6, 5);
eval_fns!(7, 6);
eval_fns!(8, 7);
eval_fns!(9, 8);
eval_fns!(10, 9);
eval_fns!(11, 10);
eval_fns!(12, 11);
eval_fns!(13, 12);
eval_fns!(14, 13);
eval_fns!(15, 14);
