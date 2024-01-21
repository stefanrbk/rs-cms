use std::{
    mem::size_of,
    process::exit,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Mutex,
    },
};

use log::{error, Level};
use rs_cms::{
    f64_to_s15_fixed16_number, f64_to_u8_fixed8_number, s15_fixed16_number_to_f64,
    state::{Context, ErrorCode, DEFAULT_CONTEXT},
    types::{Signature, XYZ},
    u8_fixed8_number_to_f64, Result, S15Fixed16Number, U16Fixed16Number, U8Fixed8Number, D50,
};

pub type TestFn = fn() -> Result<()>;

pub static REASON_TO_FAIL: Mutex<String> = Mutex::new(String::new());
pub static SUBTEST: Mutex<String> = Mutex::new(String::new());
pub static TRAPPED_ERROR: AtomicBool = AtomicBool::new(false);
pub static SIMULTANEOUS_ERRORS: AtomicUsize = AtomicUsize::new(0usize);
pub static TOTALTESTS: AtomicUsize = AtomicUsize::new(0usize);
pub static TOTALFAIL: AtomicUsize = AtomicUsize::new(0usize);

pub fn die(text: &str) -> ! {
    let mut buf = REASON_TO_FAIL.lock().unwrap();
    *buf = String::from(text);

    error!("\n{}\n", buf);
    exit(1)
}

pub fn dbg_thread() -> Context {
    Context::new(&[], None).unwrap()
}

pub fn fatal_error_quit(_ctx: &Context, _level: Level, _ec: ErrorCode, text: &str) {
    die(text)
}

pub fn reset_fatal_error() {
    DEFAULT_CONTEXT.set_error_logger(Some(fatal_error_quit))
}

pub fn fail(text: &str) {
    let mut buf = REASON_TO_FAIL.lock().unwrap();
    *buf = String::from(text);
}

pub fn subtest(text: &str) {
    let mut buf = SUBTEST.lock().unwrap();
    *buf = String::from(text);
}

pub fn clip(v: f64) -> f64 {
    v.clamp(0.0, 1.0)
}

pub fn check_base_types() -> Result<()> {
    if size_of::<u8>() != 1 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<i8>() != 1 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<u16>() != 2 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<i16>() != 2 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<u32>() != 4 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<i32>() != 4 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<u64>() != 8 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<i64>() != 8 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<f32>() != 4 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<f64>() != 8 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<Signature>() != 4 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<U8Fixed8Number>() != 2 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<S15Fixed16Number>() != 4 {
        return Err("Base type sanity check failed!");
    }
    if size_of::<U16Fixed16Number>() != 4 {
        return Err("Base type sanity check failed!");
    }

    Ok(())
}

pub fn check_quick_floor() -> Result<()> {
    if rs_cms::quick_floor(1.234) != 1 {
        die("quick_floor_word failed. Please use the \"no_fast_floor\" feature");
    }
    if rs_cms::quick_floor(32767.234) != 32767 {
        die("quick_floor_word failed. Please use the \"no_fast_floor\" feature");
    }
    if rs_cms::quick_floor(-1.234) != -2 {
        die("quick_floor_word failed. Please use the \"no_fast_floor\" feature");
    }
    if rs_cms::quick_floor(-32767.1) != -32768 {
        die("quick_floor_word failed. Please use the \"no_fast_floor\" feature");
    }

    Ok(())
}

pub fn check_quick_floor_word() -> Result<()> {
    for i in 0..u16::MAX {
        if rs_cms::quick_floor_word(i as f64 + 0.1234) != i {
            die("quick_floor_word failed. Please use the \"no_fast_floor\" feature");
        }
    }
    Ok(())
}

pub const FIXED_PRECISION_15_16: f64 = 1.0 / 65535.0;
pub const FIXED_PRECISION_8_8: f64 = 1.0 / 255.0;
pub const FLOAT_PRECISION: f32 = 1.0e-5;

pub static MAX_ERR: Mutex<f64> = Mutex::new(0.0);

pub fn is_good_val(title: &str, r#in: f64, out: f64, max: f64) -> Result<()> {
    let mut max_err = MAX_ERR.lock().unwrap();

    let err = (r#in - out).abs();

    if err > *max_err {
        *max_err = err;

        if err > max {
            let msg = &format!("({}): Must be {}, but is {}", title, r#in, out);
            fail(&msg);
            return Err("Value is outside allowed error range");
        }
    }
    Ok(())
}

pub fn is_good_fixed_15_16(title: &str, r#in: f64, out: f64) -> Result<()> {
    is_good_val(title, r#in, out, FIXED_PRECISION_15_16)
}

pub fn is_good_fixed_8_8(title: &str, r#in: f64, out: f64) -> Result<()> {
    is_good_val(title, r#in, out, FIXED_PRECISION_8_8)
}

pub fn is_good_word(title: &str, r#in: u16, out: u16) -> Result<()> {
    if r#in.abs_diff(out) > 0 {
        let msg = &format!("({}): Must be {}, but is {}", title, r#in, out);
        fail(&msg);
        return Err("Value is outside allowed error range");
    }
    Ok(())
}

pub fn is_good_word_prec(title: &str, r#in: u16, out: u16, max: u16) -> Result<()> {
    if r#in.abs_diff(out) > max {
        let msg = &format!("({}): Must be {}, but is {}", title, r#in, out);
        fail(&msg);
        return Err("Value is outside allowed error range");
    }
    Ok(())
}

fn test_single_fixed_15_16(d: f64) -> Result<()> {
    let f = f64_to_s15_fixed16_number(d);
    let round_trip = s15_fixed16_number_to_f64(f);
    let error = (d - round_trip).abs();

    if error <= FIXED_PRECISION_15_16 {
        Ok(())
    } else {
        Err("Value is outside allowed error range")
    }
}

pub fn check_fixed_point_15_16() -> Result<()> {
    test_single_fixed_15_16(1.0)?;
    test_single_fixed_15_16(2.0)?;
    test_single_fixed_15_16(1.23456)?;
    test_single_fixed_15_16(0.99999)?;
    test_single_fixed_15_16(0.1234567890123456789099999)?;
    test_single_fixed_15_16(-1.0)?;
    test_single_fixed_15_16(-2.0)?;
    test_single_fixed_15_16(-1.23456)?;
    test_single_fixed_15_16(-1.1234567890123456789099999)?;
    test_single_fixed_15_16(32767.1234567890123456789099999)?;
    test_single_fixed_15_16(-32767.1234567890123456789099999)?;

    Ok(())
}

fn test_single_fixed_8_8(d: f64) -> Result<()> {
    let f = f64_to_u8_fixed8_number(d);
    let round_trip = u8_fixed8_number_to_f64(f);
    let error = (d - round_trip).abs();

    if error <= FIXED_PRECISION_8_8 {
        Ok(())
    } else {
        Err("Value is outside allowed error range")
    }
}

pub fn check_fixed_point_8_8() -> Result<()> {
    test_single_fixed_8_8(1.0)?;
    test_single_fixed_8_8(2.0)?;
    test_single_fixed_8_8(1.23456)?;
    test_single_fixed_8_8(0.99999)?;
    test_single_fixed_8_8(0.1234567890123456789099999)?;
    test_single_fixed_8_8(255.1234567890123456789099999)?;

    Ok(())
}

pub fn check_d50_roundtrip() -> Result<()> {
    const D50_2: XYZ = XYZ {
        x: 0.96420288,
        y: 1.0,
        z: 0.82490540,
    };

    let e = (
        f64_to_s15_fixed16_number(D50.x),
        f64_to_s15_fixed16_number(D50.y),
        f64_to_s15_fixed16_number(D50.z),
    );

    let xyz = XYZ {
        x: s15_fixed16_number_to_f64(e.0),
        y: s15_fixed16_number_to_f64(e.1),
        z: s15_fixed16_number_to_f64(e.2),
    };

    let d = XYZ {
        x: (D50.x - xyz.x).abs(),
        y: (D50.y - xyz.y).abs(),
        z: (D50.z - xyz.z).abs(),
    };

    let euc = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();

    if euc > 1e-5 {
        fail(&format!("D50 roundtrip |{}|", euc));
        return Err("D50 roundtrip error outside allowed range");
    }

    let e = (
        f64_to_s15_fixed16_number(D50_2.x),
        f64_to_s15_fixed16_number(D50_2.y),
        f64_to_s15_fixed16_number(D50_2.z),
    );

    let xyz = XYZ {
        x: s15_fixed16_number_to_f64(e.0),
        y: s15_fixed16_number_to_f64(e.1),
        z: s15_fixed16_number_to_f64(e.2),
    };

    let d = XYZ {
        x: (D50_2.x - xyz.x).abs(),
        y: (D50_2.y - xyz.y).abs(),
        z: (D50_2.z - xyz.z).abs(),
    };

    let euc = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();

    if euc > 1e-5 {
        fail(&format!("D50 roundtrip |{}|", euc));
        return Err("D50 roundtrip error outside allowed range");
    }

    Ok(())
}
