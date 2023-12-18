use std::default;

use crate::{
    plugin::{lerp_flags, ParametricCurveEvaluator},
    state::Context,
    Result,
};

use super::InterpParams;

pub struct Curve {
    pub(crate) interp_params: InterpParams<u16>,
    pub(crate) segments: Box<[CurveSegment]>,
    pub(crate) seg_interp: Box<[Option<InterpParams<f32>>]>,
    pub(crate) evals: Box<[Option<ParametricCurveEvaluator>]>,
    pub(crate) table: Box<[u16]>,
}

impl Curve {
    pub(crate) fn new(
        context_id: &Context,
        segments: &[CurveSegment],
        values: &[u16],
    ) -> Result<Self> {
        let n_segments = segments.len();
        let n_entries = values.len();

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
        let mut si: Vec<Option<InterpParams<f32>>> = Vec::with_capacity(n_segments);

        let mut t: Vec<u16> = Vec::with_capacity(n_entries);

        for i in 0..n_entries {
            t.push(values[i]);
        }

        for i in 0..n_segments {
            // Type 0 is a special marker for table-based curves
            si.push(if segments[i].r#type == 0 {
                Some(InterpParams::compute(
                    context_id,
                    segments[i].n_grid_points,
                    1,
                    1,
                    Box::new([]),
                    lerp_flags::FLOAT,
                )?)
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
}

#[derive(Default, Clone)]
pub struct CurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub r#type: i32,
    pub params: [f64; 10],
    pub n_grid_points: usize,
    pub sampled_points: Box<[f32]>,
}
