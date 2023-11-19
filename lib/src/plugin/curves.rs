use super::PluginBase;

pub type ParametricCurveEvaluator = fn(r#type: i32, params: &[f64], r: f64) -> f64;

#[derive(Clone, Copy)]
pub struct CurveDef {
    pub fn_type: u32,
    pub param_count: usize,
}

pub struct ParametricCurvePlugin {
    pub base: PluginBase,
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}
