use std::{
    default,
    sync::{Arc, Mutex},
};

use crate::{
    plugin::{lerp_flags, ParametricCurveEvaluator},
    quantize_val, quick_saturate_word,
    state::Context,
    Result, MAX_NODES_IN_CURVE, MINUS_INF, PLUS_INF,
};

use super::{InterpFunction::F32, InterpParams};

pub struct Curve {
    pub(crate) interp_params: InterpParams<u16>,
    pub(crate) segments: Box<[CurveSegment]>,
    pub(crate) seg_interp: Box<[Option<Mutex<InterpParams<f32>>>]>,
    pub(crate) evals: Box<[Option<ParametricCurveEvaluator>]>,
    pub(crate) table: Box<[u16]>,
}

impl Curve {
    pub(crate) fn new(
        context_id: &Context,
        segments: &[CurveSegment],
        n_entries: usize,
        values: &[u16],
    ) -> Result<Self> {
        let n_segments = segments.len();

        // We allow huge tables, which are then restricted for smoothing operations
        if n_entries > 65530 {
            return err!(
                context_id, Error, Range, "Couldn't create tone curve of more than 65530 entries";
                str => "Too many entries");
        }

        if n_entries == 0 && n_segments == 0 {
            return err!(
                context_id, Error, Range, "Couldn't create tone curve with zero segments and no table";
                str => "No segments and no table");
        }

        let mut s: Vec<CurveSegment> = Vec::with_capacity(n_segments);
        let mut e: Vec<Option<ParametricCurveEvaluator>> = Vec::with_capacity(n_segments);
        let mut si: Vec<Option<Mutex<InterpParams<f32>>>> = Vec::with_capacity(n_segments);

        let mut t: Vec<u16> = Vec::with_capacity(n_entries);

        // Initialize members if requested
        if values.len() == n_entries {
            for i in 0..n_entries {
                t.push(values[i]);
            }
        }

        for i in 0..n_segments {
            // Type 0 is a special marker for table-based curves
            si.push(if segments[i].r#type == 0 {
                Some(Mutex::new(InterpParams::compute(
                    context_id,
                    segments[i].sampled_points.lock().unwrap().len(),
                    1,
                    1,
                    Box::new([]),
                    lerp_flags::FLOAT,
                )?))
            } else {
                None
            });
            let c = context_id.get_parametric_curve_by_type(segments[i].r#type);
            e.push(if let Some(c) = c {
                Some(c.0.eval)
            } else {
                None
            });

            s.push(segments[i].clone());
        }

        let tt = t.clone();

        let ip = InterpParams::compute(
            context_id,
            n_entries,
            1,
            1,
            t.into_boxed_slice(),
            lerp_flags::BITS_16,
        )?;

        Ok(Self {
            interp_params: ip,
            segments: s.into_boxed_slice(),
            seg_interp: si.into_boxed_slice(),
            evals: e.into_boxed_slice(),
            table: tt.into(),
        })
    }
    fn eval_segmented(&self, r: f64) -> f64 {
        for i in (0..self.segments.len()).rev() {
            // Check for domain
            if r > self.segments[i].x0 as f64 && r <= self.segments[i].x1 as f64 {
                // Type == 0 means segment is sampled
                if self.segments[i].r#type == 0 {
                    let r1 = [(r - self.segments[i].x0 as f64) as f32
                        / (self.segments[i].x1 - self.segments[i].x0)];
                    let mut out32 = [0f32];

                    // Setup the table (TODO: clean that)
                    let mut seg_interp = self.seg_interp[i].as_ref().unwrap().lock().unwrap();
                    seg_interp.table = self.segments[i].sampled_points.clone();

                    return (if let F32(func) = seg_interp.interpolation {
                        func(&r1, &mut out32, &seg_interp);
                        out32[0] as f64
                    } else {
                        self.evals[i].unwrap()(self.segments[i].r#type, self.segments[i].params, r)
                    })
                    .min(PLUS_INF)
                    .max(MINUS_INF);
                }
            }
        }

        MINUS_INF
    }

    pub fn get_estimated_table_entries(&self) -> usize {
        self.table.len()
    }

    pub fn get_estimated_table(&self) -> &[u16] {
        &self.table
    }

    pub fn build_tabulated_u16(
        context_id: &Context,
        n_entries: usize,
        values: &[u16],
    ) -> Result<Self> {
        Self::new(context_id, &[], n_entries, values)
    }

    pub fn build_segmented(context_id: &Context, segments: &[CurveSegment]) -> Result<Self> {
        let num_segs = segments.len();

        // Optimization for identity curves.
        let num_points = if num_segs == 1 && segments[0].r#type == 1 {
            entries_by_gamma(segments[0].params[0])
        } else {
            4096
        };

        let mut g = Self::new(context_id, segments, num_points, &[])?;

        // Once we have the floating point version, we can approximate a 16 bit table of 4096 entires
        // for performance reasons. This table would normally not be used except on 8/16 bit transforms.
        for i in 0..num_points {
            let r = i as f64 / (num_points as f64 - 1f64);
            let val = g.eval_segmented(r);

            // Round and saturate
            g.table[i] = quick_saturate_word(val * 65535f64);
        }

        Ok(g)
    }

    pub fn build_tabulated_f32(context_id: &Context, values: &[f32]) -> Result<Self> {
        let seg = [
            // A segmented tone curve should have function segments in the first and last positions
            // Initialize segmented curve part up to 0 to constant value = samples[0]
            CurveSegment {
                x0: MINUS_INF as f32,
                x1: 0f32,
                r#type: 6,
                params: [
                    1f64,
                    0f64,
                    0f64,
                    values[0] as f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                ],
                sampled_points: Arc::new(Mutex::new(Box::default())),
            },
            // From zero to 1
            CurveSegment {
                x0: 0f32,
                x1: 1f32,
                r#type: 0,
                params: [0f64; 10],
                sampled_points: Arc::new(Mutex::new(values.into())),
            },
            // Final segment is constant = lastsample
            CurveSegment {
                x0: 1f32,
                x1: PLUS_INF as f32,
                r#type: 6,
                params: [
                    1f64,
                    0f64,
                    0f64,
                    values[values.len() - 1] as f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                    0f64,
                ],
                sampled_points: Arc::new(Mutex::new(Box::default())),
            },
        ];

        Self::build_segmented(context_id, &seg)
    }

    pub fn build_parametric(context_id: &Context, r#type: i32, params: &[f64]) -> Result<Self> {
        if let Some((c, pos)) = context_id.get_parametric_curve_by_type(r#type) {
            let mut seg0 = CurveSegment {
                x0: MINUS_INF as f32,
                x1: PLUS_INF as f32,
                r#type,
                params: [0f64; 10],
                sampled_points: Arc::new(Mutex::new(Box::default())),
            };

            let size = c.curves[pos].param_count;
            seg0.params[..size].copy_from_slice(&params[..size]);

            Self::build_segmented(context_id, &[seg0])
        } else {
            err!(context_id, Error, UnknownExtension, "Invalid parametric curve type {}", r#type; str => "Invalid parametric curve type")
        }
    }

    pub fn build_gamma(context_id: &Context, gamma: f64) -> Result<Self> {
        Self::build_parametric(context_id, 1, &[gamma])
    }

    pub fn dup(&self) -> Result<Self> {
        Self::new(
            &self.interp_params.context_id,
            &self.segments,
            self.table.len(),
            &self.table,
        )
    }

    pub fn join(
        &self,
        y: &Self,
        context_id: &Context,
        num_resulting_points: usize,
    ) -> Result<Self> {
        let y_reversed = y.reverse_ex(num_resulting_points)?;

        let mut res = vec![0f32; num_resulting_points];

        // Iterate
        for i in 0..num_resulting_points {
            let t = i as f32 / (num_resulting_points - 1) as f32;
            let x = self.eval_f32(t);
            res[i] = y_reversed.eval_f32(x);
        }

        Self::build_tabulated_f32(context_id, &res)
    }

    pub fn reverse_ex(&self, num_resulting_points: usize) -> Result<Self> {
        let mut a = 0f64;
        let mut b = 0f64;
        // Try to reverse it analytically whenever possible
        if self.segments.len() == 1
            && self.segments[0].r#type > 0
            && self
                .interp_params
                .context_id
                .get_parametric_curve_by_type(self.segments[0].r#type)
                .is_some()
        {
            return Self::build_parametric(
                &self.interp_params.context_id,
                -self.segments[0].r#type,
                &self.segments[0].params,
            );
        }

        // Nope, reverse the table
        let mut out = Self::build_tabulated_u16(
            &self.interp_params.context_id,
            num_resulting_points,
            &vec![0u16; num_resulting_points],
        )?;

        // We want to know if this is an ascending or descending table
        let ascending = !self.is_descending();

        // Iterate across Y axis
        for i in 0..num_resulting_points {
            let y = i as f64 * 65535f64 / (num_resulting_points - 1) as f64;

            // Find interval in which y is within
            let j = get_interval(y, &self.table, &self.interp_params);
            let num_entries = self.table.len();
            if let Some(j) = j {
                // Get limits of interval
                let x1 = self.table[j];
                let x2 = self.table[j + 1];

                let y1 = (j as f64 * 65535f64) / (num_entries - 1) as f64;
                let y2 = ((j + 1) as f64 * 65535f64) / (num_entries - 1) as f64;

                // If collapsed, then use any
                if x1 == x2 {
                    out.table[i] = quick_saturate_word(if ascending { y2 } else { y1 });
                    continue;
                } else {
                    // Interpolate
                    a = (y2 - y1) / (x2 as f64 - x1 as f64);
                    b = y2 - a * x2 as f64;
                }
            }

            out.table[i] = quick_saturate_word(a * y + b);
        }

        Ok(out)
    }

    pub fn reverse(&self) -> Result<Self> {
        self.reverse_ex(4096)
    }

    pub fn smooth(&mut self, mut lambda: f64) -> Result<()> {
        let mut success_status = Ok(());
        let mut not_check = false;

        let context_id = self.interp_params.context_id.clone();

        if !self.is_linear() {
            // Only non-linear curves need smoothing
            let num_items = self.table.len();
            if num_items < MAX_NODES_IN_CURVE {
                // Allocate one more item than needed
                let mut w = vec![0f32; num_items + 1];
                let mut y = vec![0f32; num_items + 1];
                let mut z = vec![0f32; num_items + 1];

                for i in 0..num_items {
                    y[i + 1] = self.table[i] as f32;
                    w[i + 1] = 1f32;
                }

                if lambda < 0f64 {
                    not_check = true;
                    lambda = -lambda;
                }

                smooth2(&w, &y, &mut z, lambda as f32, num_items);
                // Do some reality checking...

                let mut zeros = 0usize;
                let mut poles = 0usize;
                for i in (2..=num_items).rev() {
                    if z[i] == 0f32 {
                        zeros += 1;
                    }
                    if z[i] >= 65535f32 {
                        poles += 1;
                    }
                    if z[i] < z[i - 1] {
                        success_status = if not_check {
                            Ok(())
                        } else {
                            err!(context_id, Error, Range, "Curve::smooth: Non-Monotonic.";
                                str => "Curve::smooth: Non-Monotonic.")
                        };
                    }
                }

                if success_status.is_ok() && zeros > (num_items / 3) {
                    success_status = if not_check {
                        Ok(())
                    } else {
                        err!(context_id, Error, Range, "Curve::smooth: Degenerated, mostly zeros.";
                            str => "Curve::smooth: Degenerated, mostly zeros.")
                    };
                }

                if success_status.is_ok() && poles > (num_items / 3) {
                    success_status = if not_check {
                        Ok(())
                    } else {
                        err!(context_id, Error, Range, "Curve::smooth: Degenerated, mostly poles.";
                            str => "Curve::smooth: Degenerated, mostly poles.")
                    };
                }

                if success_status.is_ok() {
                    for i in 0..num_items {
                        // Clamp to u16
                        self.table[i] = quick_saturate_word(z[i + 1] as f64);
                    }
                }
            } else {
                success_status = err!(context_id, Error, Range, "Curve::smooth: Too many points.";
                    str => "Curve::smooth: Too many points.");
            }
        }

        success_status
    }

    pub fn is_linear(&self) -> bool {
        let len = self.table.len();
        for i in 0..len {
            let diff = quantize_val(i as f64, len).abs_diff(self.table[i]);
            if diff > 0x0f {
                return false;
            }
        }
        true
    }

    pub fn is_monotonic(&self) -> bool {
        // Degenerated curves are monotonic? Ok, let's pass them
        let n = self.table.len();
        if n < 2 {
            return true;
        }

        // Curve direction
        let descending = self.is_descending();

        if descending {
            let mut last = self.table[0];

            for i in 1..n {
                if self.table[i] - last > 2
                // We allow some ripple
                {
                    return false;
                } else {
                    last = self.table[i];
                }
            }
        } else {
            let mut last = self.table[n - 1];

            for i in (0..(n - 1)).rev() {
                if self.table[i] - last > 2 {
                    return false;
                } else {
                    last = self.table[i];
                }
            }
        }

        true
    }

    pub fn is_descending(&self) -> bool {
        self.table[0] > self.table[self.table.len() - 1]
    }

    pub fn is_multisegment(&self) -> bool {
        self.segments.len() > 1
    }

    pub fn get_parametric_type(&self) -> i32 {
        if self.segments.len() != 1 {
            0
        } else {
            self.segments[0].r#type
        }
    }

    pub fn eval_f32(&self, v: f32) -> f32 {
        // Check for 16 bit table. If so, this is a limited-precision tone curve
        if self.segments.len() == 0 {
            let r#in = quick_saturate_word(v as f64 * 65535f64);
            let out = self.eval_u16(r#in);

            out as f32 / 65535f32
        } else {
            self.eval_segmented(v as f64) as f32
        }
    }

    pub fn eval_u16(&self, v: u16) -> u16 {
        let mut out = [0u16];
        if let super::InterpFunction::U16(interp) = self.interp_params.interpolation {
            interp(&[v], &mut out, &self.interp_params);
            out[0]
        } else {
            0
        }
    }

    pub fn estimate_gamma(&self, precision: f64) -> Option<f64> {
        let mut sum = 0f64;
        let mut sum2 = 0f64;
        let mut n = 0usize;

        // Excluding endpoints
        for i in 1..(MAX_NODES_IN_CURVE - 1) {
            let x = i as f64 / (MAX_NODES_IN_CURVE - 1) as f64;
            let y = self.eval_f32(x as f32) as f64;

            // Avoid 7% on lower part to prevent
            // artifacts due to linear ramps

            if y > 0f64 && y < 1f64 && x > 7e-2f64 {
                let gamma = y.ln() / x.ln();
                sum += gamma;
                sum2 += gamma * gamma;
                n += 1;
            }
        }

        // We need enough valid samples
        if n <= 1 {
            return None;
        }

        // Take a look on SD to see if gamma isn't exponential at all
        let std = ((n as f64 * sum2 - sum * sum) / (n as f64 * (n - 1) as f64)).sqrt();

        if std > precision {
            return None;
        }

        Some(sum / n as f64)
    }

    pub fn get_params(&self) -> Option<&[f64; 10]> {
        if self.segments.len() != 1 {
            None
        } else {
            Some(&self.segments[0].params)
        }
    }
}

impl Clone for Curve {
    fn clone(&self) -> Self {
        self.dup().unwrap()
    }
}

#[derive(Default, Clone)]
pub struct CurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub r#type: i32,
    pub params: [f64; 10],
    pub sampled_points: Arc<Mutex<Box<[f32]>>>,
}

fn entries_by_gamma(gamma: f64) -> usize {
    if (gamma - 1f64).abs() < 1e-3f64 {
        2usize
    } else {
        4096usize
    }
}

fn get_interval(r#in: f64, lut_table: &[u16], p: &InterpParams<u16>) -> Option<usize> {
    // A 1 point table is not allowed
    if p.domain[0] < 1 {
        return None;
    }

    // Let's see if ascending or descending
    if lut_table[0] < lut_table[p.domain[0]] {
        // Table is overall ascending
        for i in (0..p.domain[0]).rev() {
            let y0 = lut_table[i];
            let y1 = lut_table[i + 1];

            if y0 <= y1 {
                // Increasing
                if r#in >= y0 as f64 && r#in <= y1 as f64 {
                    return Some(i);
                }
            } else {
                // Decreasing
                if r#in >= y1 as f64 && r#in <= y0 as f64 {
                    return Some(i);
                }
            }
        }
    } else {
        // Table is overall descending
        for i in 0..p.domain[0] {
            let y0 = lut_table[i];
            let y1 = lut_table[i + 1];

            if y0 <= y1 {
                // Increasing
                if r#in >= y0 as f64 && r#in <= y1 as f64 {
                    return Some(i);
                }
            } else {
                if r#in >= y1 as f64 && r#in <= y0 as f64 {
                    return Some(i);
                }
            }
        }
    }

    None
}

fn smooth2(w: &[f32], y: &[f32], z: &mut [f32], lambda: f32, m: usize) {
    let mut c = [0f32; MAX_NODES_IN_CURVE];
    let mut d = [0f32; MAX_NODES_IN_CURVE];
    let mut e = [0f32; MAX_NODES_IN_CURVE];

    d[1] = w[1] + lambda;
    c[1] = -2f32 * lambda / d[1];
    e[1] = lambda / d[1];
    z[1] = w[1] * y[1];
    d[2] = w[2] + 5f32 * lambda - d[1] * c[1] * c[1];
    c[2] = (-4f32 * lambda - d[1] * c[1] * e[1]) / d[2];
    e[2] = lambda / d[2];
    z[2] = w[2] * y[2] - c[1] * z[1];

    for i in 3..(m - 1) {
        let i1 = i - 1;
        let i2 = i - 2;
        d[i] = w[i] + 6f32 * lambda - c[i1] * c[i1] * d[i1] - e[i2] * e[i2] * d[i2];
        c[i] = (-4f32 * lambda - d[i1] * c[i1] * e[i1]) / d[i];
        e[i] = lambda / d[i];
        z[i] = w[i] * y[i] - c[i1] * z[i1] - e[i2] * z[i2];
    }

    let i1 = m - 2;
    let i2 = m - 3;

    d[m - 1] = w[m - 1] + 5f32 * lambda - c[i1] * c[i1] * d[i1] - e[i2] * e[i2] * d[i2];
    c[m - 1] = (-2f32 * lambda - d[i1] * c[i1] * e[i1]) / d[m - 1];
    z[m - 1] = w[m - 1] * y[m - 1] - c[i1] * z[i1] - e[i2] * z[i2];
    let i1 = m - 1;
    let i2 = m - 2;

    d[m] = w[m] + lambda - c[i1] * c[i1] * d[i1] - e[i2] * e[i2] * d[i2];
    z[m] = (w[m] * y[m] - c[i1] * z[i1] - e[i2] * z[i2]) / d[m];
    z[m - 1] = z[m - 1] / d[m - 1] - c[m - 1] * z[m];

    for i in (1..=(m - 2)).rev() {
        z[i] = z[i] / d[i] - c[i] * z[i + 1] - e[i] * z[i + 2];
    }
}
