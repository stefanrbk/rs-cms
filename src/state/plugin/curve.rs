use crate::{state::PluginBase, MAX_TYPES_IN_PLUGIN};

pub type ParametricCurveEvaluator = fn(r#type: i32, params: [f64; 10], r: f64) -> f64;

pub struct ParametricCurvePlugin {
    pub base: PluginBase,
    pub n_fns: usize,
    pub fn_types: [u32; MAX_TYPES_IN_PLUGIN],
    pub param_count: [usize; MAX_TYPES_IN_PLUGIN],
    pub eval: ParametricCurveEvaluator,
}
