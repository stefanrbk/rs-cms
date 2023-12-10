use std::ops::Deref;

use log::info;
use rs_cms::{
    plugin::lerp_flags,
    state::{Context, DEFAULT_CONTEXT},
    types::{InterpParams, InterpFunction},
    Result,
};

use crate::helpers::fail;

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

    let p = InterpParams::compute(ctx, nodes_to_check, 1, 1, &tab, lerp_flags::BITS_16)?;
    if let InterpFunction::U16(lerp) = p.interpolation {
        for i in 0..0xffff {
            let r#in = [i as u16];
            let mut out = [0u16];
    
            lerp(&r#in, &mut out, &p);
            if down {
                out[0] = 0xffff - out[0];
            }

            if (out[0] as i32).abs_diff(r#in[0] as i32) > max_err {
                fail(&format!("({}): Must be {}, but is {}", nodes_to_check, r#in[0], out[0]));
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
    for j in 10..4096 {
        if (j % 100) == 0 {
            info!("{}", j);
        }

        check_1d(j, false, 1)?
    }

    Ok(())
}

pub fn exhaustive_check_1d_lerp_down() -> Result<()> {
    for j in 10..4096 {
        if (j % 100) == 0 {
            info!("{}", j);
        }

        check_1d(j, false, 1)?
    }

    Ok(())
}
