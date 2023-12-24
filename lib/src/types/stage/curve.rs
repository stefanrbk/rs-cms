use crate::{types::curve::Curve, state::Context};

pub struct StageCurve {
    pub curves: Box<[Curve]>,
    pub context_id: Context,
}
