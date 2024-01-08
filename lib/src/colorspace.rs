use crate::{
    err,
    flags::{HIGH_RES_PRECALC, LOW_RES_PRECALC},
    sig,
    types::Signature,
    Result,
};

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
    use sig::colorspace::*;

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
    use sig::colorspace::*;
    
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

pub(crate) fn icc_colorspace(our_notation: u32) -> Option<Signature> {
    use crate::types::pixel_type as pt;
    use sig::colorspace as sig;

    Some(match our_notation {
        1 | pt::GRAY => sig::GRAY,
        2 | pt::RGB => sig::RGB,
        pt::CMY => sig::CMY,
        pt::CMYK => sig::CMYK,
        pt::YCB_CR => sig::YCBCR,
        pt::YUV => sig::LUV,
        pt::XYZ => sig::XYZ,
        pt::LAB | pt::LAB_V2 => sig::LAB,
        pt::YUVK => sig::LUVK,
        pt::HSV => sig::HSV,
        pt::HLS => sig::HLS,
        pt::YXY => sig::YXY,
        pt::MCH1 => sig::MCH1,
        pt::MCH2 => sig::MCH2,
        pt::MCH3 => sig::MCH3,
        pt::MCH4 => sig::MCH4,
        pt::MCH5 => sig::MCH5,
        pt::MCH6 => sig::MCH6,
        pt::MCH7 => sig::MCH7,
        pt::MCH8 => sig::MCH8,
        pt::MCH9 => sig::MCH9,
        pt::MCH10 => sig::MCHA,
        pt::MCH11 => sig::MCHB,
        pt::MCH12 => sig::MCHC,
        pt::MCH13 => sig::MCHD,
        pt::MCH14 => sig::MCHE,
        pt::MCH15 => sig::MCHF,
        _ => return None,
    })
}

pub(crate) fn rcms_colorspace(profile_space: Signature) -> Option<u32> {
    use crate::types::pixel_type as pt;
    use sig::colorspace as sig;

    Some(match profile_space {
        sig::GRAY => pt::GRAY,
        sig::RGB => pt::RGB,
        sig::CMY => pt::CMY,
        sig::CMYK => pt::CMYK,
        sig::YCBCR => pt::YCB_CR,
        sig::LUV => pt::YUV,
        sig::XYZ => pt::XYZ,
        sig::LAB => pt::LAB,
        sig::LUVK => pt::YUVK,
        sig::HSV => pt::HSV,
        sig::HLS => pt::HLS,
        sig::YXY => pt::YXY,
        sig::MCH1 => pt::MCH1,
        sig::MCH2 => pt::MCH2,
        sig::MCH3 => pt::MCH3,
        sig::MCH4 => pt::MCH4,
        sig::MCH5 => pt::MCH5,
        sig::MCH6 => pt::MCH6,
        sig::MCH7 => pt::MCH7,
        sig::MCH8 => pt::MCH8,
        sig::MCH9 => pt::MCH9,
        sig::MCHA => pt::MCH10,
        sig::MCHB => pt::MCH11,
        sig::MCHC => pt::MCH12,
        sig::MCHD => pt::MCH13,
        sig::MCHE => pt::MCH14,
        sig::MCHF => pt::MCH15,
        _ => return None,
    })
}
