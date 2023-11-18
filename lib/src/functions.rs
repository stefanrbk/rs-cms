use chrono::{DateTime, Utc, TimeZone, Timelike, Datelike};

use crate::{types::DateTimeNumber, S15Fixed16Number, U16Fixed16Number, U8Fixed8Number};

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
