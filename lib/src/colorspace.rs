use crate::{
    err,
    flags::{HIGH_RES_PRECALC, LOW_RES_PRECALC},
    sig,
    types::Signature,
    Result,
};
use sig::colorspace::*;

const RGB_BLACK: [u16; 3] = [0, 0, 0];
const RGB_WHITE: [u16; 3] = [0xffff, 0xffff, 0xffff];
const CMYK_BLACK: [u16; 4] = [0xffff, 0xffff, 0xffff, 0xffff];
const CMYK_WHITE: [u16; 4] = [0, 0, 0, 0];
const LAB_BLACK: [u16; 3] = [0, 0x8080, 0x8080];
const LAB_WHITE: [u16; 3] = [0xffff, 0x8080, 0x8080];
const CMY_BLACK: [u16; 3] = [0xffff, 0xffff, 0xffff];
const CMY_WHITE: [u16; 3] = [0, 0, 0];
const GRAY_BLACK: [u16; 1] = [0];
const GRAY_WHITE: [u16; 1] = [0xffff];

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

pub(crate) fn reasonable_gridpoints_by_colorspace(colorspace: Signature, flags: u32) -> usize {
    // Already specified?
    if flags & 0x00ff0000 != 0 {
        // Yes, grab'em
        return ((flags >> 16) & 0xff) as usize;
    }

    let n_chan = channels_of(colorspace);

    // HIGH_RES_PRECALC is maximum resolution
    // LOW_RES_PRECALC is lower resolution
    match (
        flags & HIGH_RES_PRECALC != 0,
        flags & LOW_RES_PRECALC != 0,
        n_chan,
    ) {
        (_, false, 5..) => 7, // 7 for Hifi
        (true, _, 4) => 23,   // 23 for CMYK
        (true, _, _) => 49,   // 49 for RGB and others
        (_, true, 5..) => 6,  // 6 for more than 4 channels
        (_, true, 1) => 33,   // For monochrome
        (_, true, _) => 17,   // 17 for remaining
        (_, _, 4) => 17,      // 17 for CMYK
        _ => 33,              // 33 for RGB
    }
}

pub(crate) fn end_points_by_space(
    space: Signature,
) -> Option<(&'static [u16], &'static [u16], usize)> {
    // Only most common spaces
    Some(match space {
        GRAY => (&GRAY_WHITE, &GRAY_BLACK, 1),
        RGB => (&RGB_WHITE, &RGB_BLACK, 3),
        LAB => (&LAB_WHITE, &LAB_BLACK, 3),
        CMY => (&CMY_WHITE, &CMY_BLACK, 3),
        CMYK => (&CMYK_WHITE, &CMYK_BLACK, 4),
        _ => return None,
    })
}
