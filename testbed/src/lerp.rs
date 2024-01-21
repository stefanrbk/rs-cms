use std::time::{Duration, Instant};

use log::info;
use rs_cms::{
    plugin::lerp_flags,
    state::{Context, DEFAULT_CONTEXT},
    types::{InterpFunction, InterpParams, Pipeline, Stage},
    Result,
};

use crate::helpers::{dbg_thread, fail, is_good_fixed_15_16, is_good_word, MAX_ERR, FLOAT_PRECISION};

fn build_table(n: usize, tab: &mut [u16], descending: bool) {
    for i in 0..n {
        let v = (65535.0 * i as f64) / (n - 1) as f64;

        tab[if descending { n - i - 1 } else { i }] = (v + 0.5).floor() as u16;
    }
}

fn check_1d(nodes_to_check: usize, down: bool, max_err: u32) -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut tab = vec![0u16; nodes_to_check];

    build_table(nodes_to_check, &mut tab, down);

    let p = InterpParams::compute(
        ctx,
        nodes_to_check,
        1,
        1,
        tab.into_boxed_slice(),
        lerp_flags::BITS_16,
    )?;
    if let InterpFunction::U16(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i as u16];
            let mut out = [0u16];

            lerp(&r#in, &mut out, &p);
            if down {
                out[0] = 0xffff - out[0];
            }

            if (out[0] as i32).abs_diff(r#in[0] as i32) > max_err {
                fail(&format!(
                    "({}): Must be {}, but is {}",
                    nodes_to_check, r#in[0], out[0]
                ));
                return Err("Result outside range");
            }
        }

        return Ok(());
    }

    Err("Invalid InterpParams generated!")
}

pub fn check_1d_lerp_2() -> Result<()> {
    check_1d(2, false, 0)
}

pub fn check_1d_lerp_3() -> Result<()> {
    check_1d(3, false, 1)
}

pub fn check_1d_lerp_4() -> Result<()> {
    check_1d(4, false, 0)
}

pub fn check_1d_lerp_6() -> Result<()> {
    check_1d(6, false, 0)
}

pub fn check_1d_lerp_18() -> Result<()> {
    check_1d(18, false, 0)
}

pub fn check_1d_lerp_2_down() -> Result<()> {
    check_1d(2, false, 0)
}

pub fn check_1d_lerp_3_down() -> Result<()> {
    check_1d(3, false, 1)
}

pub fn check_1d_lerp_4_down() -> Result<()> {
    check_1d(4, false, 0)
}

pub fn check_1d_lerp_6_down() -> Result<()> {
    check_1d(6, false, 0)
}

pub fn check_1d_lerp_18_down() -> Result<()> {
    check_1d(18, false, 0)
}

pub fn exhaustive_check_1d_lerp() -> Result<()> {
    let mut start = Instant::now();
    for j in 10..4096 {
        let now = Instant::now();
        if (now - start) >= Duration::from_secs(2) {
            info!("{:.0}%", j as f32 / 40.96f32);
            start = now;
        }

        check_1d(j, false, 1)?
    }

    Ok(())
}

pub fn exhaustive_check_1d_lerp_down() -> Result<()> {
    let mut start = Instant::now();
    for j in 10..4096 {
        let now = Instant::now();
        if (now - start) >= Duration::from_secs(2) {
            info!("{:.0}%", j as f32 / 40.96f32);
            start = now;
        }

        check_1d(j, false, 1)?
    }

    Ok(())
}

pub fn check_3d_interpolation_f32_tetrahedral() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0f32; 3];

    let f32_table = [
        0f32, 0f32, 0f32, 0f32, 0f32, 0.25f32, 0f32, 0.5f32, 0f32, 0f32, 0.5f32, 0.25f32, 1f32,
        0f32, 0f32, 1f32, 0f32, 0.25f32, 1f32, 0.5f32, 0f32, 1f32, 0.5f32, 0.25f32,
    ];

    let p = InterpParams::compute(ctx, 2, 3, 3, Box::new(f32_table), lerp_flags::FLOAT)?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::F32(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i as f32 / 65535f32; 3];

            lerp(&r#in, &mut out, &p);

            is_good_fixed_15_16("Channel 1", out[0] as f64, r#in[0] as f64)?;
            is_good_fixed_15_16("Channel 2", out[1] as f64, (r#in[1] / 2f32) as f64)?;
            is_good_fixed_15_16("Channel 3", out[2] as f64, (r#in[2] / 4f32) as f64)?;
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn check_3d_interpolation_f32_trilinear() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0f32; 3];

    let f32_table = [
        0f32, 0f32, 0f32, 0f32, 0f32, 0.25f32, 0f32, 0.5f32, 0f32, 0f32, 0.5f32, 0.25f32, 1f32,
        0f32, 0f32, 1f32, 0f32, 0.25f32, 1f32, 0.5f32, 0f32, 1f32, 0.5f32, 0.25f32,
    ];

    let p = InterpParams::compute(
        ctx,
        2,
        3,
        3,
        Box::new(f32_table),
        lerp_flags::FLOAT | lerp_flags::TRILINEAR,
    )?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::F32(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i as f32 / 65535f32; 3];

            lerp(&r#in, &mut out, &p);

            is_good_fixed_15_16("Channel 1", out[0] as f64, r#in[0] as f64)?;
            is_good_fixed_15_16("Channel 2", out[1] as f64, (r#in[1] / 2f32) as f64)?;
            is_good_fixed_15_16("Channel 3", out[2] as f64, (r#in[2] / 4f32) as f64)?;
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn check_3d_interpolation_u16_tetrahedral() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0u16; 3];

    let u16_table = [
        0u16, 0u16, 0u16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0u16, 0u16, 0xffffu16, 0xffffu16,
        0xffffu16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0xffffu16, 0xffffu16, 0u16, 0xffffu16,
        0xffffu16, 0xffffu16,
    ];

    let p = InterpParams::compute(ctx, 2, 3, 3, Box::new(u16_table), lerp_flags::BITS_16)?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::U16(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i; 3];

            lerp(&r#in, &mut out, &p);

            is_good_word("Channel 1", out[0], r#in[0])?;
            is_good_word("Channel 2", out[1], r#in[1])?;
            is_good_word("Channel 3", out[2], r#in[2])?;
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn check_3d_interpolation_u16_trilinear() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0u16; 3];

    let u16_table = [
        0u16, 0u16, 0u16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0u16, 0u16, 0xffffu16, 0xffffu16,
        0xffffu16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0xffffu16, 0xffffu16, 0u16, 0xffffu16,
        0xffffu16, 0xffffu16,
    ];

    let p = InterpParams::compute(
        ctx,
        2,
        3,
        3,
        Box::new(u16_table),
        lerp_flags::BITS_16 | lerp_flags::TRILINEAR,
    )?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::U16(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i; 3];

            lerp(&r#in, &mut out, &p);

            is_good_word("Channel 1", out[0], r#in[0])?;
            is_good_word("Channel 2", out[1], r#in[1])?;
            is_good_word("Channel 3", out[2], r#in[2])?;
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn exhaustive_check_3d_interpolation_f32_tetrahedral() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0f32; 3];

    let f32_table = [
        0f32, 0f32, 0f32, 0f32, 0f32, 0.25f32, 0f32, 0.5f32, 0f32, 0f32, 0.5f32, 0.25f32, 1f32,
        0f32, 0f32, 1f32, 0f32, 0.25f32, 1f32, 0.5f32, 0f32, 1f32, 0.5f32, 0.25f32,
    ];

    let p = InterpParams::compute(ctx, 2, 3, 3, Box::new(f32_table), lerp_flags::FLOAT)?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::F32(lerp) = p.interpolation {
        let mut start = Instant::now();
        for r in 0..0xff {
            for g in 0..0xff {
                for b in 0..0xff {
                    let now = Instant::now();
                    if now - start >= Duration::from_secs(2) {
                        info!(
                            "{:.0}%",
                            (r * 0x10000 + g * 0x100 + b) as f64 / 0xffffff as f64 * 100f64
                        );
                        start = now
                    }
                    let r#in = [r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32];

                    lerp(&r#in, &mut out, &p);

                    is_good_fixed_15_16("Channel 1", out[0] as f64, r#in[0] as f64)?;
                    is_good_fixed_15_16("Channel 2", out[1] as f64, (r#in[1] / 2f32) as f64)?;
                    is_good_fixed_15_16("Channel 3", out[2] as f64, (r#in[2] / 4f32) as f64)?;
                }
            }
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn exhaustive_check_3d_interpolation_f32_trilinear() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0f32; 3];

    let f32_table = [
        0f32, 0f32, 0f32, 0f32, 0f32, 0.25f32, 0f32, 0.5f32, 0f32, 0f32, 0.5f32, 0.25f32, 1f32,
        0f32, 0f32, 1f32, 0f32, 0.25f32, 1f32, 0.5f32, 0f32, 1f32, 0.5f32, 0.25f32,
    ];

    let p = InterpParams::compute(
        ctx,
        2,
        3,
        3,
        Box::new(f32_table),
        lerp_flags::FLOAT | lerp_flags::TRILINEAR,
    )?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::F32(lerp) = p.interpolation {
        let mut start = Instant::now();
        for r in 0..0xff {
            for g in 0..0xff {
                for b in 0..0xff {
                    let now = Instant::now();
                    if now - start >= Duration::from_secs(2) {
                        info!(
                            "{:.0}%",
                            (r * 0x10000 + g * 0x100 + b) as f64 / 0xffffff as f64 * 100f64
                        );
                        start = now
                    }
                    let r#in = [r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32];

                    lerp(&r#in, &mut out, &p);

                    is_good_fixed_15_16("Channel 1", out[0] as f64, r#in[0] as f64)?;
                    is_good_fixed_15_16("Channel 2", out[1] as f64, (r#in[1] / 2f32) as f64)?;
                    is_good_fixed_15_16("Channel 3", out[2] as f64, (r#in[2] / 4f32) as f64)?;
                }
            }
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn exhaustive_check_3d_interpolation_u16_tetrahedral() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0u16; 3];

    let u16_table = [
        0u16, 0u16, 0u16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0u16, 0u16, 0xffffu16, 0xffffu16,
        0xffffu16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0xffffu16, 0xffffu16, 0u16, 0xffffu16,
        0xffffu16, 0xffffu16,
    ];

    let p = InterpParams::compute(ctx, 2, 3, 3, Box::new(u16_table), lerp_flags::BITS_16)?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::U16(lerp) = p.interpolation {
        let mut start = Instant::now();
        for r in 0..0xff {
            for g in 0..0xff {
                for b in 0..0xff {
                    let now = Instant::now();
                    if now - start >= Duration::from_secs(2) {
                        info!(
                            "{:.0}%",
                            (r * 0x10000 + g * 0x100 + b) as f64 / 0xffffff as f64 * 100f64
                        );
                        start = now
                    }
                    let r#in = [r as u16, g as u16, b as u16];

                    lerp(&r#in, &mut out, &p);

                    is_good_word("Channel 1", out[0], r#in[0])?;
                    is_good_word("Channel 2", out[1], r#in[1])?;
                    is_good_word("Channel 3", out[2], r#in[2])?;
                }
            }
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn exhaustive_check_3d_interpolation_u16_trilinear() -> Result<()> {
    let ctx: &Context = &DEFAULT_CONTEXT;
    let mut out = [0u16; 3];

    let u16_table = [
        0u16, 0u16, 0u16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0u16, 0u16, 0xffffu16, 0xffffu16,
        0xffffu16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0xffffu16, 0xffffu16, 0u16, 0xffffu16,
        0xffffu16, 0xffffu16,
    ];

    let p = InterpParams::compute(
        ctx,
        2,
        3,
        3,
        Box::new(u16_table),
        lerp_flags::BITS_16 | lerp_flags::TRILINEAR,
    )?;
    *MAX_ERR.lock().unwrap() = 0f64;
    if let InterpFunction::U16(lerp) = p.interpolation {
        let mut start = Instant::now();
        for r in 0..0xff {
            for g in 0..0xff {
                for b in 0..0xff {
                    let now = Instant::now();
                    if now - start >= Duration::from_secs(2) {
                        info!(
                            "{:.0}%",
                            (r * 0x10000 + g * 0x100 + b) as f64 / 0xffffff as f64 * 100f64
                        );
                        start = now
                    }
                    let r#in = [r as u16, g as u16, b as u16];

                    lerp(&r#in, &mut out, &p);

                    is_good_word("Channel 1", out[0], r#in[0])?;
                    is_good_word("Channel 2", out[1], r#in[1])?;
                    is_good_word("Channel 3", out[2], r#in[2])?;
                }
            }
        }

        let err = *MAX_ERR.lock().unwrap();
        if err > 0f64 {
            info!("|Err| {}", err);
        }

        return Ok(());
    } else {
        return Err("Invalid interpolation function");
    }
}

pub fn check_reverse_interpolation_3x3() -> Result<()> {
    let u16_table = [
        0u16, 0u16, 0u16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0u16, 0u16, 0xffffu16, 0xffffu16,
        0xffffu16, 0u16, 0u16, 0xffffu16, 0u16, 0xffffu16, 0xffffu16, 0xffffu16, 0u16, 0xffffu16,
        0xffffu16, 0xffffu16,
    ];

    let mut lut = Pipeline::new(&dbg_thread(), 3, 3)?;

    let clut = Stage::new_clut::<u16>(&dbg_thread(), 2, 3, 3, &u16_table)?;
    lut.push(clut)?;

    let mut target = [0f32; 4];
    let mut hint = [0f32; 4];
    let mut result = [0f32;4];
    lut.eval_reverse_f32(&target, &mut result, &[])?;

    if result[0] != 0.0 || result[1] != 0.0 || result[2] != 0.0 {
        let msg = "Reverse interpolation didn't find zero";
        fail(msg);
        return Err(msg);
    }

    // Transverse identity
    let mut max = 0f32;
    for i in 0..=100 {
        let r#in = i as f32 / 100.0;

        target[0] = r#in;
        target[1] = 0.0;
        target[2] = 0.0;
        lut.eval_reverse_f32(&target, &mut result, &hint)?;

        let err = (r#in - result[0]).abs();
        if err > max { max = err;}

        hint.copy_from_slice(&result);
    }

    if max <= FLOAT_PRECISION {
    Ok(())} else {
        Err("err too high")
    }
}
