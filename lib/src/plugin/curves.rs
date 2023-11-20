use crate::state::ParametricCurve;

use super::Plugin;

pub type ParametricCurveEvaluator = fn(r#type: i32, params: &[f64], r: f64) -> f64;

#[derive(Clone, Copy)]
pub struct CurveDef {
    pub fn_type: u32,
    pub param_count: usize,
}

pub struct ParametricCurvePlugin {
    pub base: Plugin,
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}

pub(crate) static DEFAULT_PARAMETRIC_CURVE: ParametricCurve = ParametricCurve {
    curves: &DEFAULT_CURVE_DEFS,
    eval: default_parametric_curve_evaluator,
};

pub(crate) fn default_parametric_curve_evaluator(_type: i32, _params: &[f64], _r: f64) -> f64 {
    0.0
}

pub(crate) static DEFAULT_CURVE_DEFS: [CurveDef; 10] = [
    CurveDef {
        fn_type: 1,
        param_count: 1,
    },
    CurveDef {
        fn_type: 2,
        param_count: 3,
    },
    CurveDef {
        fn_type: 3,
        param_count: 4,
    },
    CurveDef {
        fn_type: 4,
        param_count: 5,
    },
    CurveDef {
        fn_type: 5,
        param_count: 7,
    },
    CurveDef {
        fn_type: 6,
        param_count: 4,
    },
    CurveDef {
        fn_type: 7,
        param_count: 5,
    },
    CurveDef {
        fn_type: 8,
        param_count: 5,
    },
    CurveDef {
        fn_type: 108,
        param_count: 1,
    },
    CurveDef {
        fn_type: 109,
        param_count: 1,
    },
];
