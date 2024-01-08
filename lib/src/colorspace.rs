use crate::{err, sig, types::Signature, Result};
use sig::colorspace::*;

pub fn channels_of_colorspace(colorspace: Signature) -> Result<usize> {
    Ok(match colorspace {
        MCH1 | COLOR1 | GRAY => 1,
        MCH2 | COLOR2 => 2,
        XYZ | LAB | LUV | YCBCR | YXY | RGB | HSV | HLS | CMY | MCH3 | COLOR3 => 3,
        LUVK | CMYK | MCH4 | COLOR4 => 4,
        MCH5 | COLOR5 => 5,
        MCH6 | COLOR6 => 6,
        MCH7 | COLOR7 => 7,
        MCH8 | COLOR8 => 8,
        MCH9 | COLOR9 => 9,
        MCHA | COLOR10 => 10,
        MCHB | COLOR11 => 11,
        MCHC | COLOR12 => 12,
        MCHD | COLOR13 => 13,
        MCHE | COLOR14 => 14,
        MCHF | COLOR15 => 15,
        _ => return err!(str => "Unsupported colorspace"),
    })
}

pub(crate) fn channels_of(colorspace: Signature) -> usize {
    channels_of_colorspace(colorspace).unwrap_or(3)
}
