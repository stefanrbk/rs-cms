use crate::{quick_saturate_word, D50, LOG10E, PI};

use super::{LCh, XYZ};

const MIN_ENCODABLE_AB2: f64 = -128.0;
const MAX_ENCODABLE_AB2: f64 = (65535.0 / 256.0) - 128.0;
const MIN_ENCODABLE_AB4: f64 = -128.0;
const MAX_ENCODABLE_AB4: f64 = 127.0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LabEncoded {
    pub l: u16,
    pub a: u16,
    pub b: u16,
}

impl Lab {
    pub fn as_xyz(self, whitepoint: XYZ) -> XYZ {
        let y = (self.l + 16.0) / 116.0;
        let x = y + 0.002 * self.a;
        let z = y - 0.005 * self.b;

        let x = f_1(x) * whitepoint.x;
        let y = f_1(y) * whitepoint.y;
        let z = f_1(z) * whitepoint.z;

        XYZ { x, y, z }
    }

    pub fn as_lab_encoded_v2(self) -> LabEncoded {
        let l = clamp_l_f64_v2(self.l);
        let a = clamp_ab_f64_v2(self.a);
        let b = clamp_ab_f64_v2(self.b);

        let l = l_to_u16_v2(l);
        let a = ab_to_u16_v2(a);
        let b = ab_to_u16_v2(b);

        LabEncoded { l, a, b }
    }

    pub fn as_lab_encoded(self) -> LabEncoded {
        let l = clamp_l_f64_v4(self.l);
        let a = clamp_ab_f64_v4(self.a);
        let b = clamp_ab_f64_v4(self.b);

        let l = l_to_u16_v4(l);
        let a = ab_to_u16_v4(a);
        let b = ab_to_u16_v4(b);

        LabEncoded { l, a, b }
    }

    pub fn as_lch(self) -> LCh {
        let l = self.l;
        let c = (sqr(self.a) + sqr(self.b)).powf(0.5);
        let h = atan_to_deg(self.b, self.a);

        LCh { l, c, h }
    }

    pub fn delta_e(self, other: Self) -> f64 {
        let dl = (self.l - other.l).abs();
        let da = (self.a - other.a).abs();
        let db = (self.b - other.b).abs();

        (sqr(dl) + sqr(da) + sqr(db)).powf(0.5)
    }

    pub fn delta_e_cie_94(self, other: Self) -> f64 {
        let dl = (self.l - other.l).abs();

        let lch1 = self.as_lch();
        let lch2 = other.as_lch();

        let dc = (lch1.c - lch2.c).abs();
        let de = self.delta_e(other);

        let dhsq = sqr(de) - sqr(dl) - sqr(dc);
        let dh = if dhsq < 0.0 { 0.0 } else { dhsq.powf(0.5) };

        let c12 = (lch1.c * lch2.c).sqrt();

        let sc = 1.0 + (0.048 * c12);
        let sh = 1.0 + (0.014 * c12);

        (sqr(dl) + sqr(dc) / sqr(sc) + sqr(dh) / sqr(sh)).sqrt()
    }

    fn compute_lbfd(self) -> f64 {
        let yt = if self.l > 7.996969 {
            (sqr((self.l + 16.0) / 116.0) * ((self.l + 16.0) / 116.0)) * 100.0
        } else {
            100.0 * (self.l / 903.3)
        };

        54.6 * (LOG10E * ((yt + 1.5).log10())) - 9.6
    }

    pub fn delta_e_bfd(self, other: Self) -> f64 {
        let lbfd1 = self.compute_lbfd();
        let lbfd2 = other.compute_lbfd();
        let delta_l = lbfd2 - lbfd1;

        let lch1 = self.as_lch();
        let lch2 = other.as_lch();

        let delta_c = lch2.c - lch1.c;
        let ave_c = (lch1.c + lch2.c) / 2.0;
        let ave_h = (lch1.h + lch2.h) / 2.0;

        let de = self.delta_e(other);

        let delta_h = if sqr(de) > (sqr(other.l - self.l) + sqr(delta_c)) {
            (sqr(de) - sqr(other.l - self.l) - sqr(delta_c)).sqrt()
        } else {
            0.0
        };

        let dc = 0.035 * ave_c / (1.0 + 0.00365 * ave_c) + 0.521;
        let g = (sqr(sqr(ave_c)) / (sqr(sqr(ave_c)) + 14000.0)).sqrt();
        let t = 0.627
            + (0.055 * ((ave_h - 254.0) / (180.0 / PI)).cos()
                - 0.040 * ((2.0 * ave_h - 136.0) / (180.0 / PI)).cos()
                + 0.070 * ((3.0 * ave_h - 31.0) / (180.0 / PI)).cos()
                + 0.049 * ((4.0 * ave_h + 114.0) / (180.0 / PI)).cos()
                - 0.015 * ((5.0 * ave_h - 103.0) / (180.0 / PI)).cos());

        let dh = dc * (g * t + 1.0 - g);
        let rh = -0.260 * ((ave_h - 308.0) / (180.0 / PI)).cos()
            - 0.379 * ((2.0 * ave_h - 160.0) / (180.0 / PI)).cos()
            - 0.636 * ((3.0 * ave_h + 254.0) / (180.0 / PI)).cos()
            + 0.226 * ((4.0 * ave_h + 140.0) / (180.0 / PI)).cos()
            - 0.194 * ((5.0 * ave_h + 280.0) / (180.0 / PI)).cos();

        let rc = ((ave_c * ave_c * ave_c * ave_c * ave_c * ave_c)
            / ((ave_c * ave_c * ave_c * ave_c * ave_c * ave_c) + 70000000.0))
            .sqrt();
        let rt = rh * rc;

        (sqr(delta_l)
            + sqr(delta_c / dc)
            + sqr(delta_h / dh)
            + (rt * (delta_c / dc) * (delta_h / dh)))
            .sqrt()
    }

    pub fn delta_e_cmc(self, other: Lab, l: f64, c: f64) -> f64 {
        if self.l == 0.0 && other.l == 0.0 {
            return 0.0;
        }

        let lch1 = self.as_lch();
        let lch2 = other.as_lch();

        let dl = other.l - self.l;
        let dc = lch2.c - lch1.c;

        let de = self.delta_e(other);

        let dh = if sqr(de) > sqr(dl) + sqr(dc) {
            (sqr(de) - sqr(dl) - sqr(dc)).sqrt()
        } else {
            0.0
        };

        let t = if (lch1.h > 164.0) && (lch1.h < 345.0) {
            0.56 + (0.2 * ((lch1.h + 168.0) / (180.0 / PI)).cos()).abs()
        } else {
            0.36 + (0.4 * ((lch1.h + 35.0) / (180.0 / PI)).cos()).abs()
        };

        let sc = 0.0638 * lch1.c / (1.0 + 0.0131 * lch1.c) + 0.638;
        let sl = if self.l < 16.0 {
            0.511
        } else {
            0.040975 * self.l / (1.0 + 0.01765 * self.l)
        };

        let f = ((lch1.c * lch1.c * lch1.c * lch1.c)
            / ((lch1.c * lch1.c * lch1.c * lch1.c) + 1900.0))
            .sqrt();
        let sh = sc * (t * f + 1.0 - f);

        (sqr(dl / (l * sl)) + sqr(dc / (c * sc)) + sqr(dh / sh)).sqrt()
    }

    pub fn delta_e_cie_2000(self, other: Lab, kl: f64, kc: f64, kh: f64) -> f64 {
        let l1 = self.l;
        let a1 = self.a;
        let b1 = self.b;
        let c = (sqr(a1) + sqr(b1)).sqrt();

        let ls = other.l;
        let r#as = other.a;
        let bs = other.b;
        let cs = (sqr(r#as) + sqr(bs)).sqrt();

        let g = 0.5
            * (1.0
                - (((c + cs) / 2.0).powf(7.0) / (((c + cs) / 2.0).powf(7.0) + 25.0f64.powf(7.0)))
                    .sqrt());

        let a_p = (1.0 + g) * a1;
        let b_p = b1;
        let c_p = (sqr(a_p) + sqr(b_p)).sqrt();
        let h_p = atan_to_deg(b_p, a_p);

        let a_ps = (1.0 + g) * r#as;
        let b_ps = bs;
        let c_ps = (sqr(a_ps) + sqr(b_ps)).sqrt();
        let h_ps = atan_to_deg(b_ps, a_ps);

        let mean_c_p = (c_p + c_ps) / 2.0;

        let hps_plus_hp = h_ps + h_p;
        let hps_minus_hp = h_ps - h_p;

        let meanh_p = if hps_minus_hp.abs() <= 180.000001 {
            (hps_plus_hp) / 2.0
        } else {
            if hps_plus_hp < 360.0 {
                (hps_plus_hp + 360.0) / 2.0
            } else {
                (hps_plus_hp - 360.0) / 2.0
            }
        };

        let delta_h = if hps_minus_hp <= -180.000001 {
            hps_minus_hp + 360.0
        } else {
            if hps_minus_hp > 180.0 {
                hps_minus_hp - 360.0
            } else {
                hps_minus_hp
            }
        };
        let delta_l = ls - l1;
        let delta_c = c_ps - c_p;

        let delta_h = 2.0 * (c_ps * c_p).sqrt() * (radians(delta_h) / 2.0).sin();

        let t = 1.0 - 0.17 * (radians(meanh_p - 30.0)).cos()
            + 0.24 * (radians(2.0 * meanh_p)).cos()
            + 0.32 * (radians(3.0 * meanh_p + 6.0)).cos()
            - 0.2 * (radians(4.0 * meanh_p - 63.0)).cos();

        let sl = 1.0
            + (0.015 * sqr((ls + l1) / 2.0 - 50.0)) / (20.0 + sqr((ls + l1) / 2.0 - 50.0)).sqrt();

        let sc = 1.0 + 0.045 * (c_p + c_ps) / 2.0;
        let sh = 1.0 + 0.015 * ((c_ps + c_p) / 2.0) * t;

        let delta_ro = 30.0 * (-sqr((meanh_p - 275.0) / 25.0)).exp();

        let rc =
            2.0 * (((mean_c_p).powf(7.0)) / ((mean_c_p).powf(7.0) + (25.0f64).powf(7.0))).sqrt();

        let rt = -(2.0 * radians(delta_ro)).sin() * rc;

        (sqr(delta_l / (sl * kl))
            + sqr(delta_c / (sc * kc))
            + sqr(delta_h / (sh * kh))
            + rt * (delta_c / (sc * kc)) * (delta_h / (sh * kh)))
            .sqrt()
    }
}

impl LabEncoded {
    pub fn as_lab_v2(self) -> Lab {
        let l = l_to_f64_v2(self.l);
        let a = ab_to_f64_v2(self.a);
        let b = ab_to_f64_v2(self.b);

        Lab { l, a, b }
    }
    pub fn as_lab(self) -> Lab {
        let l = l_to_f64_v4(self.l);
        let a = ab_to_f64_v4(self.a);
        let b = ab_to_f64_v4(self.b);

        Lab { l, a, b }
    }
}

fn f_1(t: f64) -> f64 {
    const LIMIT: f64 = 24.0 / 116.0;

    if t <= LIMIT {
        (108.0 / 841.0) * (t - (16.0 / 116.0))
    } else {
        t * t * t
    }
}

fn l_to_f64_v2(v: u16) -> f64 {
    v as f64 / 652.8
}

fn ab_to_f64_v2(v: u16) -> f64 {
    (v as f64 / 256.0) - 128.0
}

fn l_to_f64_v4(v: u16) -> f64 {
    v as f64 / 655.35
}

fn ab_to_f64_v4(v: u16) -> f64 {
    (v as f64 / 257.0) - 128.0
}

fn l_to_u16_v2(l: f64) -> u16 {
    quick_saturate_word(l * 652.8)
}

fn ab_to_u16_v2(ab: f64) -> u16 {
    quick_saturate_word((ab + 128.0) * 256.0)
}

fn clamp_l_f64_v2(l: f64) -> f64 {
    const L_MAX: f64 = (0xffffu16 as f64 * 100.0) / 0xff00u16 as f64;

    l.clamp(0.0, L_MAX)
}

fn clamp_ab_f64_v2(ab: f64) -> f64 {
    ab.clamp(MIN_ENCODABLE_AB2, MAX_ENCODABLE_AB2)
}

fn clamp_l_f64_v4(l: f64) -> f64 {
    l.clamp(0.0, 100.0)
}

fn clamp_ab_f64_v4(ab: f64) -> f64 {
    ab.clamp(MIN_ENCODABLE_AB4, MAX_ENCODABLE_AB4)
}

fn l_to_u16_v4(l: f64) -> u16 {
    quick_saturate_word(l * 655.35)
}

fn ab_to_u16_v4(ab: f64) -> u16 {
    quick_saturate_word((ab + 128.0) * 257.0)
}

fn sqr(v: f64) -> f64 {
    v * v
}

fn radians(deg: f64) -> f64 {
    (deg * PI) / 180.0
}

fn atan_to_deg(a: f64, b: f64) -> f64 {
    let mut h = if a == 0.0 && b == 0.0 {
        0.0
    } else {
        a.atan2(b)
    } * (180.0 * PI);

    while h > 360.0 {
        h -= 360.0;
    }

    while h < 0.0 {
        h += 360.0;
    }

    h
}
