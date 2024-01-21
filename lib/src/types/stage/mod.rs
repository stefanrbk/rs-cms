use std::any::{Any, TypeId};

use log::info;

use crate::{
    from_f32_to_u16, from_u16_to_f32,
    plugin::lerp_flags,
    quantize_val, quick_saturate_word, sig,
    state::Context,
    types::{stage::curve::StageCurve, Lab, XYZ},
    Result, Sampler, MAX_CHANNELS, MAX_INPUT_DIMENSIONS, MAX_STAGE_CHANNELS, SAMPLER_INSPECT,
};

use self::{clut::StageCLut, matrix::StageMatrix};

use super::{curve::Curve, InterpFunction::F32, InterpFunction::U16, InterpParams, Signature};

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

    fn eval_matrix(&self, r#in: &[f32], out: &mut [f32]) {
        if let Some(data) = self.data.downcast_ref::<StageMatrix>() {
            // Input is already in 0..1.0 domain
            for i in 0..self.out_chans {
                let mut tmp = 0f64;
                for j in 0..self.in_chans {
                    tmp += r#in[j] as f64 * data.double[i * self.in_chans + j] as f64;
                }

                if data.offset.len() != 0 {
                    tmp += data.offset[i];
                }

                out[i] = tmp as f32;
            }
        }
        // Output in 0..1.0 domain
    }

    fn dup_matrix(&self) -> Result<Box<dyn Any>> {
        let data = self
            .data
            .downcast_ref::<StageMatrix>()
            .ok_or("Stage is not a Matrix")?;

        Ok(Box::new(StageMatrix {
            double: data.double.clone(),
            offset: data.offset.clone(),
        }))
    }

    pub fn new_matrix(
        context_id: &Context,
        rows: usize,
        cols: usize,
        matrix: &[f64],
        offset: &[f64],
    ) -> Result<Self> {
        // check for overflow
        let n = rows
            .checked_mul(cols)
            .ok_or("new_matrix rows and cols overflowed!")?;

        let new_elem = StageMatrix {
            double: matrix[..n].into(),
            offset: if offset.len() == 0 {
                Box::default()
            } else {
                offset[..rows].into()
            },
        };

        Ok(Self::new(
            &context_id,
            sig::mpe_stage::MATRIX,
            cols,
            rows,
            Self::eval_matrix,
            Self::dup_matrix,
            Box::new(new_elem),
        ))
    }

    fn eval_clut_f32(&self, r#in: &[f32], out: &mut [f32]) {
        if let Some(data) = self.data.downcast_ref::<StageCLut<f32>>() {
            if let F32(interp) = data.params.interpolation {
                interp(r#in, out, &data.params);
            }
        }
    }

    fn eval_clut_u16(&self, r#in: &[f32], out: &mut [f32]) {
        if let Some(data) = self.data.downcast_ref::<StageCLut<u16>>() {
            if let U16(interp) = data.params.interpolation {
                let mut in16 = [0u16; MAX_STAGE_CHANNELS];
                let mut out16 = [0u16; MAX_STAGE_CHANNELS];

                from_f32_to_u16(&r#in[..self.in_chans], &mut in16[..self.in_chans]);
                interp(&in16, &mut out16, &data.params);
                from_u16_to_f32(&out16[..self.out_chans], &mut out[..self.out_chans]);
            }
        }
    }

    pub fn dup_clut<T>(&self) -> Result<Box<dyn Any>>
    where
        T: Copy + 'static,
    {
        let data = self
            .data
            .downcast_ref::<StageCLut<T>>()
            .ok_or("Stage is not a CLut")?;

        let params = InterpParams::compute_ex(
            &self.context_id,
            &data.params.n_samples,
            data.params.n_inputs,
            data.params.n_outputs,
            data.tab.clone(),
            data.params.flags,
        )?;

        Ok(Box::new(StageCLut {
            tab: data.tab.clone(),
            params,
        }))
    }

    pub fn new_clut_granular<T: Copy + 'static>(
        context_id: &Context,
        clut_points: &[usize],
        in_chan: usize,
        out_chan: usize,
        table: &[T],
    ) -> Result<Self> {
        if in_chan > MAX_INPUT_DIMENSIONS {
            return err!(context_id, Error, Range, "Too many input channels ({} channels, max={}", in_chan, MAX_INPUT_DIMENSIONS;
                str => "Too many input channels");
        }

        let params = InterpParams::compute_ex(
            context_id,
            clut_points,
            in_chan,
            out_chan,
            table.into(),
            if table.type_id() == TypeId::of::<[u16]>() {
                lerp_flags::BITS_16
            } else if table.type_id() == TypeId::of::<[f32]>() {
                lerp_flags::FLOAT
            } else {
                return err!(context_id, Error, NotSuitable, "Invalid table type (expected &[f32] or &[u16], found {:?})", table.type_id();
                    str => "Invalid table type");
            },
        )?;

        let data = Box::new(StageCLut {
            tab: table.into(),
            params,
        });

        Ok(Stage::new(
            context_id,
            sig::mpe_stage::CLUT,
            in_chan,
            out_chan,
            Self::eval_clut_u16,
            Self::dup_clut::<T>,
            data,
        ))
    }

    pub fn new_clut<T: Copy + 'static>(
        context_id: &Context,
        grid_points: usize,
        in_chan: usize,
        out_chan: usize,
        table: &[u16],
    ) -> Result<Self> {
        let dims = [grid_points; MAX_INPUT_DIMENSIONS];

        Self::new_clut_granular(context_id, &dims, in_chan, out_chan, table)
    }

    pub(crate) fn new_identity_clut(context_id: &Context, num_chans: usize) -> Result<Self> {
        let dims = [2usize; MAX_INPUT_DIMENSIONS];

        let mut mpe = Self::new_clut_granular::<u16>(context_id, &dims, num_chans, num_chans, &[])?;

        mpe.sample_clut_u16(identity_sampler, &num_chans, 0)?;
        mpe.implements = sig::mpe_stage::IDENTITY;

        Ok(mpe)
    }

    pub fn sample_clut_u16(
        &mut self,
        sampler: Sampler<u16>,
        cargo: &dyn Any,
        flags: u32,
    ) -> Result<()> {
        let r#in = &mut [0u16; MAX_INPUT_DIMENSIONS + 1];
        let out = &mut [0u16; MAX_STAGE_CHANNELS];

        let clut = self
            .data
            .downcast_mut::<StageCLut<u16>>()
            .ok_or("Stage doesn't contain StageCLut data!")?;

        let n_samples = clut.params.n_samples;
        let n_inputs = clut.params.n_inputs;
        let n_outputs = clut.params.n_outputs;

        if n_inputs > MAX_INPUT_DIMENSIONS
            || n_inputs <= 0
            || n_outputs > MAX_STAGE_CHANNELS
            || n_outputs <= 0
        {
            return err!(str => "Invalid params");
        }

        let n_total_points = cube_size(&n_samples[..n_inputs]);
        if n_total_points == 0 {
            return err!(str => "Invalid point calculation");
        }

        let mut index = 0usize;
        for i in 0..n_total_points {
            let mut rest = i;
            for t in (0..n_inputs).rev() {
                let colorant = rest % n_samples[t];

                rest /= n_samples[t];

                r#in[t] = quantize_val(colorant as f64, n_samples[t]);
            }

            for t in 0..n_outputs {
                out[t] = clut.tab[index + t];
            }

            sampler(r#in, out, cargo)?;

            if flags & SAMPLER_INSPECT == 0 {
                for t in 0..n_outputs {
                    clut.tab[index + t] = out[t];
                }
            }

            index += n_outputs;
        }

        Ok(())
    }

    pub fn sample_clut_f32(
        &mut self,
        sampler: Sampler<f32>,
        cargo: &dyn Any,
        flags: u32,
    ) -> Result<()> {
        let r#in = &mut [0f32; MAX_INPUT_DIMENSIONS + 1];
        let out = &mut [0f32; MAX_STAGE_CHANNELS];

        let clut = self
            .data
            .downcast_mut::<StageCLut<f32>>()
            .ok_or("Stage doesn't contain StageCLut data!")?;

        let n_samples = clut.params.n_samples;
        let n_inputs = clut.params.n_inputs;
        let n_outputs = clut.params.n_outputs;

        if n_inputs > MAX_INPUT_DIMENSIONS
            || n_inputs <= 0
            || n_outputs > MAX_STAGE_CHANNELS
            || n_outputs <= 0
        {
            return err!(str => "Invalid params");
        }

        let n_total_points = cube_size(&n_samples[..n_inputs]);
        if n_total_points == 0 {
            return err!(str => "Invalid point calculation");
        }

        let mut index = 0usize;
        for i in 0..n_total_points {
            let mut rest = i;
            for t in (0..n_inputs).rev() {
                let colorant = rest % n_samples[t];

                rest /= n_samples[t];

                r#in[t] = (quantize_val(colorant as f64, n_samples[t]) as f64 / 65535f64) as f32;
            }

            for t in 0..n_outputs {
                out[t] = clut.tab[index + t];
            }

            sampler(r#in, out, cargo)?;

            if flags & SAMPLER_INSPECT == 0 {
                for t in 0..n_outputs {
                    clut.tab[index + t] = out[t];
                }
            }

            index += n_outputs;
        }

        Ok(())
    }

    fn eval_lab_to_xyz(&self, r#in: &[f32], out: &mut [f32]) {
        const XYZ_ADJ: f64 = XYZ::MAX;

        // V4 rules
        let l = r#in[0] as f64 * 100.0;
        let a = r#in[1] as f64 * 255.0 - 128.0;
        let b = r#in[2] as f64 * 255.0 - 128.0;

        let lab = Lab { l, a, b };

        let xyz = lab.as_xyz_d50();

        // From XYZ, range 0..19997 to 0..1.0, note that 1.99997 comes from 0xfff
        // encoded as 1.15 fixed point, so 1 + (32767.0 / 32768.0)

        out[0] = (xyz.x / XYZ_ADJ) as f32;
        out[1] = (xyz.y / XYZ_ADJ) as f32;
        out[2] = (xyz.z / XYZ_ADJ) as f32;
    }

    fn dup_null(&self) -> Result<Box<dyn Any>> {
        Ok(Box::new(0u8))
    }

    pub(crate) fn new_lab_to_xyz(context_id: &Context) -> Result<Self> {
        Ok(Self::new(
            context_id,
            sig::mpe_stage::LAB_2_XYZ,
            3,
            3,
            Self::eval_lab_to_xyz,
            Self::dup_null,
            Box::new(0u8),
        ))
    }

    pub(crate) fn new_lab_v2_to_v4_curves(context_id: &Context) -> Result<Self> {
        let mut lab_table = [
            Curve::build_tabulated_u16(context_id, 258, &[])?,
            Curve::build_tabulated_u16(context_id, 258, &[])?,
            Curve::build_tabulated_u16(context_id, 258, &[])?,
        ];

        for j in 0..3 {
            // We need to map * (0xffff / 0xff00), that's same as (257 / 256)
            // So we can use 258-entry tables to do the trick (i / 257) * (255 * 257) * (257 / 256)
            for i in 0..257 {
                lab_table[j].table[i] = ((i as u32 * 0xffff + 0x80) >> 8) as u16;
            }

            lab_table[j].table[257] = 0xffff;
        }

        let mut mpe = Stage::new_curves(context_id, &lab_table)?;
        mpe.implements = sig::mpe_stage::LAB_V2_TO_V4;

        Ok(mpe)
    }

    pub(crate) fn new_lab_v2_to_v4(context_id: &Context) -> Result<Self> {
        const N: f64 = 65535.0 / 65280.0;
        const V2_TO_V4: [f64; 9] = [N, 0.0, 0.0, 0.0, N, 0.0, 0.0, 0.0, N];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &V2_TO_V4, &[])?;
        mpe.implements = sig::mpe_stage::LAB_V2_TO_V4;

        Ok(mpe)
    }

    pub(crate) fn new_lab_v4_to_v2(context_id: &Context) -> Result<Self> {
        const N: f64 = 65280.0 / 65535.0;
        const V4_TO_V2: [f64; 9] = [N, 0.0, 0.0, 0.0, N, 0.0, 0.0, 0.0, N];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &V4_TO_V2, &[])?;
        mpe.implements = sig::mpe_stage::LAB_V4_TO_V2;

        Ok(mpe)
    }

    pub(crate) fn normalize_from_lab_f64(context_id: &Context) -> Result<Self> {
        const N: f64 = 1.0 / 100.0;
        const NN: f64 = 1.0 / 255.0;
        const A1: [f64; 9] = [N, 0.0, 0.0, 0.0, NN, 0.0, 0.0, 0.0, NN];

        const O1: [f64; 3] = [0.0, 128.0 / 255.0, 128.0 / 255.0];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &A1, &O1)?;
        mpe.implements = sig::mpe_stage::LAB_2_FLOAT_PCS;

        Ok(mpe)
    }

    pub(crate) fn normalize_from_xyz_f64(context_id: &Context) -> Result<Self> {
        const N: f64 = 32768.0 / 65535.0;
        const A1: [f64; 9] = [N, 0.0, 0.0, 0.0, N, 0.0, 0.0, 0.0, N];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &A1, &[])?;
        mpe.implements = sig::mpe_stage::XYZ_2_FLOAT_PCS;

        Ok(mpe)
    }

    pub(crate) fn normalize_to_lab_f64(context_id: &Context) -> Result<Self> {
        const N: f64 = 100.0;
        const NN: f64 = 255.0;
        const A1: [f64; 9] = [N, 0.0, 0.0, 0.0, NN, 0.0, 0.0, 0.0, NN];

        const O1: [f64; 3] = [0.0, -128.0, -128.0];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &A1, &O1)?;
        mpe.implements = sig::mpe_stage::FLOAT_PCS_2_LAB;

        Ok(mpe)
    }

    pub(crate) fn normalize_to_xyz_f64(context_id: &Context) -> Result<Self> {
        const N: f64 = 65535.0 / 32768.0;
        const A1: [f64; 9] = [N, 0.0, 0.0, 0.0, N, 0.0, 0.0, 0.0, N];

        let mut mpe = Stage::new_matrix(context_id, 3, 3, &A1, &[])?;
        mpe.implements = sig::mpe_stage::FLOAT_PCS_2_XYZ;

        Ok(mpe)
    }

    fn clipper(&self, r#in: &[f32], out: &mut [f32]) {
        for i in 0..self.in_chans {
            out[i] = r#in[i].max(0.0);
        }
    }

    pub(crate) fn new_clip_negatives(context_id: &Context, num_chans: usize) -> Result<Self> {
        Ok(Self::new(
            context_id,
            sig::mpe_stage::CLIP_NEGATIVES,
            num_chans,
            num_chans,
            Self::clipper,
            Self::dup_null,
            Box::new(0u8),
        ))
    }

    fn eval_xyz_to_lab(&self, r#in: &[f32], out: &mut [f32]) {
        const XYZ_ADJ: f64 = XYZ::MAX;

        // From 0..1.0 to XYZ
        let x = r#in[0] as f64 * XYZ_ADJ;
        let y = r#in[1] as f64 * XYZ_ADJ;
        let z = r#in[2] as f64 * XYZ_ADJ;

        let xyz = XYZ { x, y, z };

        let lab = xyz.as_lab_d50();

        // From V4 Lab to 0..1.0

        out[0] = (lab.l / 100.0) as f32;
        out[1] = ((lab.a + 128.0) / 255.0) as f32;
        out[2] = ((lab.b + 128.0) / 255.0) as f32;
    }

    pub(crate) fn new_xyz_to_lab(context_id: &Context) -> Result<Self> {
        Ok(Self::new(
            context_id,
            sig::mpe_stage::XYZ_2_LAB,
            3,
            3,
            Self::eval_xyz_to_lab,
            Self::dup_null,
            Box::new(0u8),
        ))
    }

    pub(crate) fn new_lab_prelin(context_id: &Context) -> Result<Self> {
        let params = [2.4];
        let lab_table = [
            Curve::build_gamma(context_id, 1.0)?,
            Curve::build_parametric(context_id, 108, &params)?,
            Curve::build_parametric(context_id, 108, &params)?,
        ];

        Stage::new_curves(context_id, &lab_table)
    }

    pub fn input_channels(&self) -> usize {
        self.in_chans
    }

    pub fn output_channels(&self) -> usize {
        self.out_chans
    }

    pub fn r#type(&self) -> Signature {
        self.r#type
    }

    pub fn data(&self) -> &dyn Any {
        &self.data
    }

    pub fn context_id(&self) -> &Context {
        &self.context_id
    }

    pub fn dup(&self) -> Result<Self> {
        Ok(Self::new(
            &self.context_id,
            self.r#type,
            self.in_chans,
            self.out_chans,
            self.eval,
            self.dup,
            (self.dup)(self)?,
        ))
    }
}

fn cube_size(dims: &[usize]) -> usize {
    let mut b = dims.len();
    let mut rv = 1usize;

    loop {
        if b <= 0 {
            break;
        }

        let dim = dims[b - 1];
        if dim <= 1 {
            return 0; // Error
        }

        let rv1 = rv.checked_mul(dim);
        if let Some(rv1) = rv1 {
            rv = rv1;
        } else {
            return 0; // Error}
        }

        b -= 1;
    }

    rv
}

fn identity_sampler(r#in: &[u16], out: &mut [u16], cargo: &dyn Any) -> Result<()> {
    let n_chan = cargo
        .downcast_ref::<usize>()
        .ok_or("cargo must be a `usize`")?;

    for i in 0..*n_chan {
        out[i] = r#in[i];
    }

    Ok(())
}

pub fn slice_space_u16(
    n_inputs: usize,
    clut_points: &[usize],
    sampler: Sampler<u16>,
    cargo: &dyn Any,
) -> Result<()> {
    let mut r#in = [0u16; MAX_CHANNELS];

    let n_total_points = cube_size(&clut_points[..n_inputs]);

    for i in 0..n_total_points {
        let mut rest = i;
        for t in (0..n_inputs).rev() {
            let colorant = rest % clut_points[t];

            rest /= clut_points[t];
            r#in[t] = quantize_val(colorant as f64, clut_points[t]);
        }

        sampler(&r#in, &mut [], cargo)?;
    }

    Ok(())
}

pub fn slice_space_f32(
    n_inputs: usize,
    clut_points: &[usize],
    sampler: Sampler<f32>,
    cargo: &dyn Any,
) -> Result<()> {
    let mut r#in = [0f32; MAX_CHANNELS];

    let n_total_points = cube_size(&clut_points[..n_inputs]);

    for i in 0..n_total_points {
        let mut rest = i;
        for t in (0..n_inputs).rev() {
            let colorant = rest % clut_points[t];

            rest /= clut_points[t];
            r#in[t] = (quantize_val(colorant as f64, clut_points[t]) as f64 / 65535f64) as f32;
        }

        sampler(&r#in, &mut [], cargo)?;
    }

    Ok(())
}

mod clut;
mod curve;
mod matrix;
