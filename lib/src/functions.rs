use std::mem::size_of;

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

use crate::{
    types::DateTimeNumber, S15Fixed16Number, U16Fixed16Number, U8Fixed8Number, PTR_ALIGNMENT,
};

#[inline]
pub fn u8_fixed8_number_to_f64(fixed8: U8Fixed8Number) -> f64 {
    let lsb = fixed8 & 0xFF;
    let msb = fixed8 >> 8;

    msb as f64 + (lsb as f64 / 255.0)
}

#[inline]
pub fn f64_to_u8_fixed8_number(val: f64) -> U8Fixed8Number {
    let tmp = f64_to_s15_fixed16_number(val);
    ((tmp >> 8) & 0xFFFF) as U8Fixed8Number
}

#[inline]
pub fn s15_fixed16_number_to_f64(fix32: S15Fixed16Number) -> f64 {
    let sign = if fix32 < 0 { -1.0 } else { 1.0 };
    let fix32 = fix32.abs();

    let whole = ((fix32 >> 16) & 0xFFFF) as u16;
    let frac_part = (fix32 & 0xFFFF) as u16;

    let mid = frac_part as f64 / 65536.0;
    let floater = whole as f64 + mid;

    floater * sign
}

#[inline]
pub fn u16_fixed16_number_to_f64(fix32: U16Fixed16Number) -> f64 {
    fix32 as f64 / 65536.0
}

#[inline]
pub fn f64_to_s15_fixed16_number(v: f64) -> S15Fixed16Number {
    f64::floor((v * 65536.0) + 0.5) as S15Fixed16Number
}

#[inline]
pub fn f64_to_u16_fixed16_number(v: f64) -> U16Fixed16Number {
    f64::floor((v * 65536.0) + 0.5) as U16Fixed16Number
}

#[inline]
pub fn encode_date_time(source: DateTime<Utc>) -> DateTimeNumber {
    DateTimeNumber {
        seconds: (source.second() as u16).to_be(),
        minutes: (source.minute() as u16).to_be(),
        hours: (source.hour() as u16).to_be(),
        day: (source.day() as u16).to_be(),
        month: (source.month() as u16).to_be(),
        year: (source.year() as u16).to_be(),
    }
}

#[inline]
pub fn decode_date_time(source: DateTimeNumber) -> DateTime<Utc> {
    let utc = Utc;
    utc.with_ymd_and_hms(
        source.year.to_be() as i32,
        source.month.to_be() as u32,
        source.day.to_be() as u32,
        source.hours.to_be() as u32,
        source.minutes.to_be() as u32,
        source.seconds.to_be() as u32,
    )
    .unwrap()
}

#[inline]
pub const fn align_long(x: usize) -> usize {
    (x + (size_of::<u32>() - 1)) & !(size_of::<u32>() - 1)
}
#[inline]
pub const fn align_mem(x: usize) -> usize {
    x + (PTR_ALIGNMENT - 1) & !(PTR_ALIGNMENT - 1)
}

#[inline]
pub const fn from_8_to_16(rgb: u8) -> u16 {
    ((rgb as u16) << 8) | rgb as u16
}

#[inline]
pub const fn from_16_to_8(rgb: u16) -> u8 {
    ((((rgb as u32) * 65281 + 8388608) >> 24) & 0xFF) as u8
}

#[inline]
pub const fn fixed_to_int(x: S15Fixed16Number) -> i32 {
    x >> 16
}

#[inline]
pub const fn fixed_rest_to_int(x: S15Fixed16Number) -> i32 {
    x & 0xFFFF
}

#[inline]
pub const fn round_fixed_to_int(x: S15Fixed16Number) -> i32 {
    (x + 0x8000) >> 16
}

#[inline]
pub const fn to_fixed_domain(a: i32) -> S15Fixed16Number {
    a + ((a + 0x7fff) / 0xffff)
}

#[inline]
pub const fn from_fixed_domain(a: S15Fixed16Number) -> i32 {
    a - ((a + 0x7fff) >> 16)
}

#[inline]
pub fn quick_floor(val: f64) -> i32 {
    if cfg!(no_fast_floor) {
        return val.floor() as i32;
    }
    const DOUBLE_2_FIX_MAGIC: f64 = 68719476736.0 * 1.5;

    union Split {
        pub val: f64,
        pub halves: [i32; 2],
    }

    let i = Split {
        val: val + DOUBLE_2_FIX_MAGIC,
    };

    unsafe_block!("Accessing part of a union for performing quick_floor" => i.halves[0] >> 16)
}

#[inline]
pub fn quick_floor_word(d: f64) -> u16 {
    (quick_floor(d - 32767.0) + 32767) as u16
}

#[inline]
pub fn quick_saturate_word(d: f64) -> u16 {
    let d = d + 0.5;
    if d <= 0.0 {
        return 0;
    }
    if d >= 65535.0 {
        return 0xffff;
    }

    quick_floor_word(d)
}

pub fn quantize_val(i: f64, max_samples: usize) -> u16 {
    let x = (i as f64 * 65535f64) / (max_samples - 1) as f64;
    quick_saturate_word(x)
}
