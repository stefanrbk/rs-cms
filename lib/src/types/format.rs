use bitfield::bitfield;

bitfield! {
    pub struct Format(u32);
    pub u8, bytes, set_bytes: 2, 0;
    pub u8, channels, set_channels: 6, 3;
    pub u8, extra, set_extra: 9, 7;
    pub bool, doswap, set_doswap: 10;
    pub bool, endian16, set_endian16: 11;
    pub bool, planar, set_planar: 12;
    pub bool, flavor, set_flavor: 13;
    pub bool, swapfirst, set_swapfirst: 14;
    pub u8, colorspace, set_colorspace: 20, 16;
    pub bool, optimized, set_optimized: 21;
    pub bool, float, set_float: 22;
    pub bool, premul, set_premul: 23;
}

impl Format {
    pub const GRAY_8: Format =
        Format(colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(1));
    pub const GRAY_8_REV: Format =
        Format(colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(1) | flavor_sh(1));
    pub const GRAY_16: Format =
        Format(colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(2));
    pub const GRAY_16_REV: Format =
        Format(colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(2) | flavor_sh(1));
    pub const GRAY_16_SE: Format =
        Format(colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(2) | endian16_sh(1));
    pub const GRAYA_8: Format =
        Format(colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1));
    pub const GRAYA_8_PREMUL: Format = Format(
        colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | premul_sh(1),
    );
    pub const GRAYA_16: Format =
        Format(colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2));
    pub const GRAYA_16_PREMUL: Format = Format(
        colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | premul_sh(1),
    );
    pub const GRAYA_16_SE: Format = Format(
        colorspace_sh(pixel_type::GRAY)
            | extra_sh(1)
            | channels_sh(1)
            | bytes_sh(2)
            | endian16_sh(1),
    );
    pub const GRAYA_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | planar_sh(1),
    );
    pub const GRAYA_16_PLANAR: Format = Format(
        colorspace_sh(pixel_type::GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | planar_sh(1),
    );

    pub const RGB_8: Format = Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(1));
    pub const RGB_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const BGR_8: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1));
    pub const BGR_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | planar_sh(1),
    );
    pub const RGB_16: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2));
    pub const RGB_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const RGB_16_SE: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));
    pub const BGR_16: Format =
        Format(colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1));
    pub const BGR_16_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | planar_sh(1),
    );
    pub const BGR_16_SE: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );

    pub const RGBA_8: Format =
        Format(colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1));
    pub const RGBA_8_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | premul_sh(1),
    );
    pub const RGBA_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | planar_sh(1),
    );
    pub const RGBA_16: Format =
        Format(colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2));
    pub const RGBA_16_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | premul_sh(1),
    );
    pub const RGBA_16_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | planar_sh(1),
    );
    pub const RGBA_16_SE: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | endian16_sh(1),
    );

    pub const ARGB_8: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | swapfirst_sh(1),
    );
    pub const ARGB_8_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | swapfirst_sh(1)
            | premul_sh(1),
    );
    pub const ARGB_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | swapfirst_sh(1)
            | planar_sh(1),
    );
    pub const ARGB_16: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | swapfirst_sh(1),
    );
    pub const ARGB_16_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | swapfirst_sh(1)
            | premul_sh(1),
    );

    pub const ABGR_8: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1),
    );
    pub const ABGR_8_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | doswap_sh(1)
            | premul_sh(1),
    );
    pub const ABGR_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | doswap_sh(1)
            | planar_sh(1),
    );
    pub const ABGR_16: Format = Format(
        colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1),
    );
    pub const ABGR_16_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | premul_sh(1),
    );
    pub const ABGR_16_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | planar_sh(1),
    );
    pub const ABGR_16_SE: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );

    pub const BGRA_8: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | doswap_sh(1)
            | swapfirst_sh(1),
    );
    pub const BGRA_8_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | doswap_sh(1)
            | swapfirst_sh(1)
            | premul_sh(1),
    );
    pub const BGRA_8_PLANAR: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(1)
            | doswap_sh(1)
            | swapfirst_sh(1)
            | planar_sh(1),
    );
    pub const BGRA_16: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | swapfirst_sh(1),
    );
    pub const BGRA_16_PREMUL: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | swapfirst_sh(1)
            | premul_sh(1),
    );
    pub const BGRA_16_SE: Format = Format(
        colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | endian16_sh(1)
            | doswap_sh(1)
            | swapfirst_sh(1),
    );

    pub const CMY_8: Format = Format(colorspace_sh(pixel_type::CMY) | channels_sh(3) | bytes_sh(1));
    pub const CMY_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::CMY) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const CMY_16: Format =
        Format(colorspace_sh(pixel_type::CMY) | channels_sh(3) | bytes_sh(2));
    pub const CMY_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::CMY) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const CMY_16_SE: Format =
        Format(colorspace_sh(pixel_type::CMY) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));

    pub const CMYK_8: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(1));
    pub const CMYKA_8: Format =
        Format(colorspace_sh(pixel_type::CMYK) | extra_sh(1) | channels_sh(4) | bytes_sh(1));
    pub const CMYK_8_REV: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(1) | flavor_sh(1));
    pub const YUVK_8: Format = Format::CMYK_8_REV;
    pub const CMYK_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(1) | planar_sh(1));
    pub const CMYK_16: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2));
    pub const CMYK_16_REV: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2) | flavor_sh(1));
    pub const YUVK_16: Format = Format::CMYK_16_REV;
    pub const CMYK_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2) | planar_sh(1));
    pub const CMYK_16_SE: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2) | endian16_sh(1));

    pub const KYMC_8: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC_16: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC_16_SE: Format = Format(
        colorspace_sh(pixel_type::CMYK)
            | channels_sh(4)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );

    pub const KCMY_8: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(1) | swapfirst_sh(1));
    pub const KCMY_8_REV: Format = Format(
        colorspace_sh(pixel_type::CMYK)
            | channels_sh(4)
            | bytes_sh(1)
            | flavor_sh(1)
            | swapfirst_sh(1),
    );
    pub const KCMY_16: Format =
        Format(colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2) | swapfirst_sh(1));
    pub const KCMY_16_REV: Format = Format(
        colorspace_sh(pixel_type::CMYK)
            | channels_sh(4)
            | bytes_sh(2)
            | flavor_sh(1)
            | swapfirst_sh(1),
    );
    pub const KCMY_16_SE: Format = Format(
        colorspace_sh(pixel_type::CMYK)
            | channels_sh(4)
            | bytes_sh(2)
            | endian16_sh(1)
            | swapfirst_sh(1),
    );

    pub const CMYK5_8: Format =
        Format(colorspace_sh(pixel_type::MCH5) | channels_sh(5) | bytes_sh(1));
    pub const CMYK5_16: Format =
        Format(colorspace_sh(pixel_type::MCH5) | channels_sh(5) | bytes_sh(2));
    pub const CMYK5_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH5) | channels_sh(5) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC5_8: Format =
        Format(colorspace_sh(pixel_type::MCH5) | channels_sh(5) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC5_16: Format =
        Format(colorspace_sh(pixel_type::MCH5) | channels_sh(5) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC5_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH5)
            | channels_sh(5)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK6_8: Format =
        Format(colorspace_sh(pixel_type::MCH6) | channels_sh(6) | bytes_sh(1));
    pub const CMYK6_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::MCH6) | channels_sh(6) | bytes_sh(1) | planar_sh(1));
    pub const CMYK6_16: Format =
        Format(colorspace_sh(pixel_type::MCH6) | channels_sh(6) | bytes_sh(2));
    pub const CMYK6_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::MCH6) | channels_sh(6) | bytes_sh(2) | planar_sh(1));
    pub const CMYK6_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH6) | channels_sh(6) | bytes_sh(2) | endian16_sh(1));
    pub const CMYK7_8: Format =
        Format(colorspace_sh(pixel_type::MCH7) | channels_sh(7) | bytes_sh(1));
    pub const CMYK7_16: Format =
        Format(colorspace_sh(pixel_type::MCH7) | channels_sh(7) | bytes_sh(2));
    pub const CMYK7_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH7) | channels_sh(7) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC7_8: Format =
        Format(colorspace_sh(pixel_type::MCH7) | channels_sh(7) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC7_16: Format =
        Format(colorspace_sh(pixel_type::MCH7) | channels_sh(7) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC7_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH7)
            | channels_sh(7)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK8_8: Format =
        Format(colorspace_sh(pixel_type::MCH8) | channels_sh(8) | bytes_sh(1));
    pub const CMYK8_16: Format =
        Format(colorspace_sh(pixel_type::MCH8) | channels_sh(8) | bytes_sh(2));
    pub const CMYK8_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH8) | channels_sh(8) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC8_8: Format =
        Format(colorspace_sh(pixel_type::MCH8) | channels_sh(8) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC8_16: Format =
        Format(colorspace_sh(pixel_type::MCH8) | channels_sh(8) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC8_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH8)
            | channels_sh(8)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK9_8: Format =
        Format(colorspace_sh(pixel_type::MCH9) | channels_sh(9) | bytes_sh(1));
    pub const CMYK9_16: Format =
        Format(colorspace_sh(pixel_type::MCH9) | channels_sh(9) | bytes_sh(2));
    pub const CMYK9_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH9) | channels_sh(9) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC9_8: Format =
        Format(colorspace_sh(pixel_type::MCH9) | channels_sh(9) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC9_16: Format =
        Format(colorspace_sh(pixel_type::MCH9) | channels_sh(9) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC9_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH9)
            | channels_sh(9)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK10_8: Format =
        Format(colorspace_sh(pixel_type::MCH10) | channels_sh(10) | bytes_sh(1));
    pub const CMYK10_16: Format =
        Format(colorspace_sh(pixel_type::MCH10) | channels_sh(10) | bytes_sh(2));
    pub const CMYK10_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH10) | channels_sh(10) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC10_8: Format =
        Format(colorspace_sh(pixel_type::MCH10) | channels_sh(10) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC10_16: Format =
        Format(colorspace_sh(pixel_type::MCH10) | channels_sh(10) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC10_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH10)
            | channels_sh(10)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK11_8: Format =
        Format(colorspace_sh(pixel_type::MCH11) | channels_sh(11) | bytes_sh(1));
    pub const CMYK11_16: Format =
        Format(colorspace_sh(pixel_type::MCH11) | channels_sh(11) | bytes_sh(2));
    pub const CMYK11_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH11) | channels_sh(11) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC11_8: Format =
        Format(colorspace_sh(pixel_type::MCH11) | channels_sh(11) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC11_16: Format =
        Format(colorspace_sh(pixel_type::MCH11) | channels_sh(11) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC11_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH11)
            | channels_sh(11)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );
    pub const CMYK12_8: Format =
        Format(colorspace_sh(pixel_type::MCH12) | channels_sh(12) | bytes_sh(1));
    pub const CMYK12_16: Format =
        Format(colorspace_sh(pixel_type::MCH12) | channels_sh(12) | bytes_sh(2));
    pub const CMYK12_16_SE: Format =
        Format(colorspace_sh(pixel_type::MCH12) | channels_sh(12) | bytes_sh(2) | endian16_sh(1));
    pub const KYMC12_8: Format =
        Format(colorspace_sh(pixel_type::MCH12) | channels_sh(12) | bytes_sh(1) | doswap_sh(1));
    pub const KYMC12_16: Format =
        Format(colorspace_sh(pixel_type::MCH12) | channels_sh(12) | bytes_sh(2) | doswap_sh(1));
    pub const KYMC12_16_SE: Format = Format(
        colorspace_sh(pixel_type::MCH12)
            | channels_sh(12)
            | bytes_sh(2)
            | doswap_sh(1)
            | endian16_sh(1),
    );

    // Colorimetric
    pub const XYZ_16: Format =
        Format(colorspace_sh(pixel_type::XYZ) | channels_sh(3) | bytes_sh(2));

    pub const LAB_8: Format = Format(colorspace_sh(pixel_type::LAB) | channels_sh(3) | bytes_sh(1));
    pub const LAB_V2_8: Format =
        Format(colorspace_sh(pixel_type::LAB_V2) | channels_sh(3) | bytes_sh(1));

    pub const ALAB_8: Format = Format(
        colorspace_sh(pixel_type::LAB)
            | channels_sh(3)
            | bytes_sh(1)
            | extra_sh(1)
            | swapfirst_sh(1),
    );
    pub const ALAB_V2_8: Format = Format(
        colorspace_sh(pixel_type::LAB_V2)
            | channels_sh(3)
            | bytes_sh(1)
            | extra_sh(1)
            | swapfirst_sh(1),
    );
    pub const LAB_16: Format =
        Format(colorspace_sh(pixel_type::LAB) | channels_sh(3) | bytes_sh(2));
    pub const LAB_V2_16: Format =
        Format(colorspace_sh(pixel_type::LAB_V2) | channels_sh(3) | bytes_sh(2));
    pub const YXY_16: Format =
        Format(colorspace_sh(pixel_type::YXY) | channels_sh(3) | bytes_sh(2));

    // YCbCr
    pub const YCB_CR_8: Format =
        Format(colorspace_sh(pixel_type::YCB_CR) | channels_sh(3) | bytes_sh(1));

    pub const YCB_CR_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::YCB_CR) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const YCB_CR_16: Format =
        Format(colorspace_sh(pixel_type::YCB_CR) | channels_sh(3) | bytes_sh(2));
    pub const YCB_CR_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::YCB_CR) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const YCB_CR_16_SE: Format =
        Format(colorspace_sh(pixel_type::YCB_CR) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));

    // YUV
    pub const YUV_8: Format = Format(colorspace_sh(pixel_type::YUV) | channels_sh(3) | bytes_sh(1));

    pub const YUV_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::YUV) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const YUV_16: Format =
        Format(colorspace_sh(pixel_type::YUV) | channels_sh(3) | bytes_sh(2));
    pub const YUV_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::YUV) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const YUV_16_SE: Format =
        Format(colorspace_sh(pixel_type::YUV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));

    // HLS
    pub const HLS_8: Format = Format(colorspace_sh(pixel_type::HLS) | channels_sh(3) | bytes_sh(1));

    pub const HLS_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::HLS) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const HLS_16: Format =
        Format(colorspace_sh(pixel_type::HLS) | channels_sh(3) | bytes_sh(2));
    pub const HLS_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::HLS) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const HLS_16_SE: Format =
        Format(colorspace_sh(pixel_type::HLS) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));

    // HSV
    pub const HSV_8: Format = Format(colorspace_sh(pixel_type::HSV) | channels_sh(3) | bytes_sh(1));

    pub const HSV_8_PLANAR: Format =
        Format(colorspace_sh(pixel_type::HSV) | channels_sh(3) | bytes_sh(1) | planar_sh(1));
    pub const HSV_16: Format =
        Format(colorspace_sh(pixel_type::HSV) | channels_sh(3) | bytes_sh(2));
    pub const HSV_16_PLANAR: Format =
        Format(colorspace_sh(pixel_type::HSV) | channels_sh(3) | bytes_sh(2) | planar_sh(1));
    pub const HSV_16_SE: Format =
        Format(colorspace_sh(pixel_type::HSV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1));

    // Named color index. Only 16 bits is allowed (don't check colorspace)
    pub const NAMED_COLOR_INDEX: Format = Format(channels_sh(1) | bytes_sh(2));

    // Float formatters.
    pub const XYZ_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::XYZ) | channels_sh(3) | bytes_sh(4));

    pub const LAB_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::LAB) | channels_sh(3) | bytes_sh(4));
    pub const LAB_A_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::LAB) | extra_sh(1) | channels_sh(3) | bytes_sh(4),
    );
    pub const GRAY_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(4));
    pub const GRAYA_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(4) | extra_sh(1),
    );
    pub const GRAYA_FLT_PREMUL: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::GRAY)
            | channels_sh(1)
            | bytes_sh(4)
            | extra_sh(1)
            | premul_sh(1),
    );
    pub const RGB_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(4));

    pub const RGBA_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4),
    );
    pub const RGBA_FLT_PREMUL: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | premul_sh(1),
    );
    pub const ARGB_FLT: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | swapfirst_sh(1),
    );
    pub const ARGB_FLT_PREMUL: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | swapfirst_sh(1)
            | premul_sh(1),
    );
    pub const BGR_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(4) | doswap_sh(1),
    );
    pub const BGRA_FLT: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | doswap_sh(1)
            | swapfirst_sh(1),
    );
    pub const BGRA_FLT_PREMUL: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | doswap_sh(1)
            | swapfirst_sh(1)
            | premul_sh(1),
    );
    pub const ABGR_FLT: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | doswap_sh(1),
    );
    pub const ABGR_FLT_PREMUL: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(4)
            | doswap_sh(1)
            | premul_sh(1),
    );

    pub const CMYK_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(4));

    // Floating point formatters.
    // NOTE THAT 'BYTES' FIELD IS SET TO ZERO ON DLB because 8 bytes overflows the bitfield
    pub const XYZ_DBL: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::XYZ) | channels_sh(3) | bytes_sh(0));

    pub const LAB_DBL: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::LAB) | channels_sh(3) | bytes_sh(0));
    pub const GRAY_DBL: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(0));
    pub const RGB_DBL: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(0));
    pub const BGR_DBL: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(0) | doswap_sh(1),
    );
    pub const CMYK_DBL: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(0));

    // IEEE 754-2008 "half"
    pub const GRAY_HALF_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::GRAY) | channels_sh(1) | bytes_sh(2));

    pub const RGB_HALF_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2));
    pub const RGBA_HALF_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2),
    );
    pub const CMYK_HALF_FLT: Format =
        Format(float_sh(1) | colorspace_sh(pixel_type::CMYK) | channels_sh(4) | bytes_sh(2));

    pub const ARGB_HALF_FLT: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | swapfirst_sh(1),
    );
    pub const BGR_HALF_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1),
    );
    pub const BGRA_HALF_FLT: Format = Format(
        float_sh(1)
            | colorspace_sh(pixel_type::RGB)
            | extra_sh(1)
            | channels_sh(3)
            | bytes_sh(2)
            | doswap_sh(1)
            | swapfirst_sh(1),
    );
    pub const ABGR_HALF_FLT: Format = Format(
        float_sh(1) | colorspace_sh(pixel_type::RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1),
    );
}

#[inline(always)]
pub const fn premul_sh(m: u32) -> u32 {
    m << 23
}

#[inline(always)]
pub const fn float_sh(m: u32) -> u32 {
    m << 22
}

#[inline(always)]
pub const fn optimized_sh(m: u32) -> u32 {
    m << 21
}

#[inline(always)]
pub const fn colorspace_sh(m: u32) -> u32 {
    m << 16
}

#[inline(always)]
pub const fn swapfirst_sh(m: u32) -> u32 {
    m << 14
}

#[inline(always)]
pub const fn flavor_sh(m: u32) -> u32 {
    m << 13
}

#[inline(always)]
pub const fn planar_sh(m: u32) -> u32 {
    m << 12
}

#[inline(always)]
pub const fn endian16_sh(m: u32) -> u32 {
    m << 11
}

#[inline(always)]
pub const fn doswap_sh(m: u32) -> u32 {
    m << 10
}

#[inline(always)]
pub const fn extra_sh(m: u32) -> u32 {
    m << 7
}

#[inline(always)]
pub const fn channels_sh(m: u32) -> u32 {
    m << 3
}

#[inline(always)]
pub const fn bytes_sh(m: u32) -> u32 {
    m << 0
}

pub mod pixel_type {
    pub const ANY: u32 = 0;
    pub const GRAY: u32 = 3;
    pub const RGB: u32 = 4;
    pub const CMY: u32 = 5;
    pub const CMYK: u32 = 6;
    pub const YCB_CR: u32 = 7;
    pub const YUV: u32 = 8;
    pub const XYZ: u32 = 9;
    pub const LAB: u32 = 10;
    pub const YUVK: u32 = 11;
    pub const HSV: u32 = 12;
    pub const HLS: u32 = 13;
    pub const YXY: u32 = 14;
    pub const MCH1: u32 = 15;
    pub const MCH2: u32 = 16;
    pub const MCH3: u32 = 17;
    pub const MCH4: u32 = 18;
    pub const MCH5: u32 = 19;
    pub const MCH6: u32 = 20;
    pub const MCH7: u32 = 21;
    pub const MCH8: u32 = 22;
    pub const MCH9: u32 = 23;
    pub const MCH10: u32 = 24;
    pub const MCH11: u32 = 25;
    pub const MCH12: u32 = 26;
    pub const MCH13: u32 = 27;
    pub const MCH14: u32 = 28;
    pub const MCH15: u32 = 29;
    pub const LAB_V2: u32 = 30;
}
