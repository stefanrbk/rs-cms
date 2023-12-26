use crate::{state::ParametricCurve, MATRIX_DET_TOLERANCE, PLUS_INF};

use super::Plugin;

pub type ParametricCurveEvaluator = fn(r#type: i32, params: [f64; 10], r: f64) -> f64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CurveDef {
    pub fn_type: u32,
    pub param_count: usize,
}

pub struct ParametricCurvePlugin {
    pub base: Plugin,
    pub curves: &'static [CurveDef],
    pub eval: ParametricCurveEvaluator,
}

pub(crate) const DEFAULT_PARAMETRIC_CURVE: ParametricCurve = ParametricCurve {
    curves: &DEFAULT_CURVE_DEFS,
    eval: default_parametric_curve_evaluator,
};

pub(crate) fn default_parametric_curve_evaluator(r#type: i32, params: [f64; 10], r: f64) -> f64 {
    match r#type {
        // X = Y ^ Gamma
        1 => {
            if r < 0f64 {
                if (params[0] - 1.0).abs() < MATRIX_DET_TOLERANCE {
                    r
                } else {
                    0f64
                }
            } else {
                r.powf(params[0])
            }
        }
        // Type 1 Reversed: X = Y ^1/gamma
        -1 => {
            if r < 0f64 {
                if (params[0] - 1.0).abs() < MATRIX_DET_TOLERANCE {
                    r
                } else {
                    0f64
                }
            } else {
                if params[0].abs() < MATRIX_DET_TOLERANCE {
                    PLUS_INF
                } else {
                    r.powf(1f64 / params[0])
                }
            }
        }
        // CIE 122-1966
        // Y = (aX + b)^Gamma  | X >= -b/a
        // Y = 0               | else
        2 => {
            if params[1].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                let disc = -params[2] / params[1];
                if r >= disc {
                    let e = params[1] * r + params[2];

                    if e > 0f64 {
                        e.powf(params[0])
                    } else {
                        0f64
                    }
                } else {
                    0f64
                }
            }
        }
        // Type 2 Reversed
        // X = (Y ^1/g  - b) / a
        -2 => {
            if params[0].abs() < MATRIX_DET_TOLERANCE || params[1].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                (if r < 0f64 {
                    0f64
                } else {
                    (r.powf(1f64 / params[0]) - params[2]) / params[1]
                })
                .max(0f64)
            }
        }
        // IEC 61966-3
        // Y = (aX + b)^Gamma + c | X <= -b/a
        // Y = c                  | else
        3 => {
            if params[1].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                let disc = (-params[2] / params[1]).max(0f64);

                if r >= disc {
                    let e = params[1] * r + params[2];

                    if e > 0f64 {
                        e.powf(params[0]) + params[3]
                    } else {
                        0f64
                    }
                } else {
                    params[3]
                }
            }
        }
        // Type 3 reversed
        // X=((Y-c)^1/g - b)/a      | (Y>=c)
        // X=-b/a                   | (Y<c)
        -3 => {
            if params[0].abs() < MATRIX_DET_TOLERANCE || params[1].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                if r >= params[3] {
                    let e = r - params[3];

                    if e > 0f64 {
                        (e.powf(1f64 / params[0]) - params[2]) / params[1]
                    } else {
                        0f64
                    }
                } else {
                    -params[2] / params[1]
                }
            }
        }
        // IEC 61966-2.1 (sRGB)
        // Y = (aX + b)^Gamma | X >= d
        // Y = cX             | X < d
        4 => {
            if r >= params[4] {
                let e = params[1] * r + params[2];

                if e > 0f64 {
                    e.powf(params[0])
                } else {
                    0f64
                }
            } else {
                r * params[3]
            }
        }
        // Type 4 reversed
        // X=((Y^1/g-b)/a)    | Y >= (ad+b)^g
        // X=Y/c              | Y< (ad+b)^g
        -4 => {
            let e = params[1] * params[4] + params[2];

            let disc = if e < 0f64 { 0f64 } else { e.powf(params[0]) };

            if r >= disc {
                if params[0].abs() < MATRIX_DET_TOLERANCE || params[1].abs() < MATRIX_DET_TOLERANCE
                {
                    0f64
                } else {
                    (r.powf(1f64 / params[0]) - params[2]) / params[1]
                }
            } else {
                if params[3].abs() < MATRIX_DET_TOLERANCE {
                    0f64
                } else {
                    r / params[3]
                }
            }
        }
        // Y = (aX + b)^Gamma + e | X >= d
        // Y = cX + f             | X < d
        5 => {
            if r >= params[4] {
                let e = params[1] * r + params[2];

                if e > 0f64 {
                    e.powf(params[0]) + params[5]
                } else {
                    params[5]
                }
            } else {
                r * params[3] + params[6]
            }
        }
        // Reversed type 5
        // X=((Y-e)1/g-b)/a   | Y >=(ad+b)^g+e), cd+f
        // X=(Y-f)/c          | else
        -5 => {
            let disc = params[3] * params[4] * params[6];
            if r >= disc {
                let e = r - params[5];
                if e < 0f64 {
                    0f64
                } else {
                    if params[0].abs() < MATRIX_DET_TOLERANCE
                        || params[1].abs() < MATRIX_DET_TOLERANCE
                    {
                        0f64
                    } else {
                        (e.powf(1f64 / params[0]) - params[2]) / params[1]
                    }
                }
            } else {
                if params[3].abs() < MATRIX_DET_TOLERANCE {
                    0f64
                } else {
                    (r - params[6]) / params[3]
                }
            }
        }
        // Types 6,7,8 comes from segmented curves as described in ICCSpecRevision_02_11_06_Float.pdf
        // Type 6 is basically identical to type 5 without d

        // Y = (a * X + b) ^ Gamma + c
        6 => {
            let e = params[1] * r + params[2];

            if e < 0f64 {
                params[3]
            } else {
                e.powf(params[0]) + params[3]
            }
        }
        // ((Y - c) ^1/Gamma - b) / a
        -6 => {
            if params[0].abs() < MATRIX_DET_TOLERANCE || params[1].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                let e = r - params[3];
                if e < 0f64 {
                    0f64
                } else {
                    (e.powf(1f64 / params[0]) - params[2]) / params[1]
                }
            }
        }
        // Y = a * log (b * X^Gamma + c) + d
        7 => {
            let e = params[2] * r.powf(params[0]) + params[3];
            if e <= 0f64 {
                params[4]
            } else {
                params[1] * e.log10() + params[4]
            }
        }
        // (Y - d) / a = log(b * X ^Gamma + c)
        // pow(10, (Y-d) / a) = b * X ^Gamma + c
        // pow((pow(10, (Y-d) / a) - c) / b, 1/g) = X
        -7 => {
            if params[0].abs() < MATRIX_DET_TOLERANCE
                || params[1].abs() < MATRIX_DET_TOLERANCE
                || params[2].abs() < MATRIX_DET_TOLERANCE
            {
                0f64
            } else {
                ((10f64.powf((r - params[4]) / params[1]) - params[3]) / params[2])
                    .powf(1f64 / params[0])
            }
        }
        //Y = a * b^(c*X+d) + e
        8 => params[0] * params[1].powf(params[2] * r + params[3]) + params[4],
        // Y = (log((y-e) / a) / log(b) - d ) / c
        // a=0, b=1, c=2, d=3, e=4,
        -8 => {
            let disc = r - params[4];
            if disc < 0f64 {
                0f64
            } else {
                if params[0].abs() < MATRIX_DET_TOLERANCE || params[2].abs() < MATRIX_DET_TOLERANCE
                {
                    0f64
                } else {
                    ((disc / params[0]).ln() / params[1].ln() - params[3]) / params[2]
                }
            }
        }
        // S-Shaped: (1 - (1-x)^1/g)^1/g
        108 => {
            if params[0].abs() < MATRIX_DET_TOLERANCE {
                0f64
            } else {
                (1f64 - (1f64 - r).powf(1f64 / params[0])).powf(1f64 / params[0])
            }
        }
        // y = (1 - (1-x)^1/g)^1/g
        // y^g = (1 - (1-x)^1/g)
        // 1 - y^g = (1-x)^1/g
        // (1 - y^g)^g = 1 - x
        // 1 - (1 - y^g)^g
        -108 => 1f64 - (1f64 - r.powf(params[0])).powf(params[0]),
        // Sigmoidals
        109 => sigmoid_factory(params[0], r),
        -109 => inverted_sigmoid_factory(params[0], r),
        // Unsupported parametric curve. Should never reach here
        _ => 0f64,
    };
    todo!()
}

pub(crate) const DEFAULT_CURVE_DEFS: &[CurveDef] = &[
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

#[inline(always)]
fn sigmoid_base(k: f64, t: f64) -> f64 {
    (1f64 / (1f64 + (-k * t).exp())) - 0.5f64
}

#[inline(always)]
fn inverted_sigmoid_base(k: f64, t: f64) -> f64 {
    -((1f64 / (t + 0.5f64)) - 1f64).ln() / k
}

#[inline(always)]
fn sigmoid_factory(k: f64, t: f64) -> f64 {
    let correction = 0.5f64 / sigmoid_base(k, 1f64);

    (correction * sigmoid_base(k, (2f64 * t) - 1f64)) + 0.5f64
}

#[inline(always)]
fn inverted_sigmoid_factory(k: f64, t: f64) -> f64 {
    let correction = 0.5f64 / sigmoid_base(k, 1f64);

    (inverted_sigmoid_base(k, (t - 0.5f64) / correction) + 1f64) / 2f64
}
