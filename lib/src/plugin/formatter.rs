use crate::types::Transform;

use super::Plugin;

pub type FormatterIn16 =
    for<'a> fn(cargo: Transform, values: &'a mut [u16], buffer: &'a [u8], stride: u32) -> &'a [u8];
pub type FormatterInFloat =
    for<'a> fn(cargo: Transform, values: &'a mut [f32], buffer: &'a [u8], stride: u32) -> &'a [u8];

pub type FormatterOut16 = for<'a> fn(
    cargo: Transform,
    values: &'a [u16],
    buffer: &'a mut [u8],
    stride: u32,
) -> &'a mut [u8];
pub type FormatterOutFloat = for<'a> fn(
    cargo: Transform,
    values: &'a [f32],
    buffer: &'a mut [u8],
    stride: u32,
) -> &'a mut [u8];

pub type FormatterInFactory = fn(r#type: u32, flags: u32) -> FormatterIn;
pub type FormatterOutFactory = fn(r#type: u32, flags: u32) -> FormatterOut;

pub struct FormatterPlugin {
    pub base: Plugin,
    pub in_factory: FormatterInFactory,
    pub out_factory: FormatterOutFactory,
}

pub enum FormatterIn {
    F32(Option<FormatterInFloat>),
    U16(Option<FormatterIn16>),
}

impl FormatterIn {
    pub const fn is_f32(&self) -> bool {
        matches!(*self, Self::F32(_))
    }
    pub const fn is_u16(&self) -> bool {
        matches!(*self, Self::U16(_))
    }
    pub fn is_f32_and(self, f: impl FnOnce(Option<FormatterInFloat>) -> bool) -> bool {
        match self {
            Self::U16(_) => false,
            Self::F32(x) => f(x),
        }
    }
    pub fn is_u16_and(self, f: impl FnOnce(Option<FormatterIn16>) -> bool) -> bool {
        match self {
            Self::U16(x) => f(x),
            Self::F32(_) => false,
        }
    }
}

pub enum FormatterOut {
    F32(Option<FormatterOutFloat>),
    U16(Option<FormatterOut16>),
}

impl FormatterOut {
    pub const fn is_f32(&self) -> bool {
        matches!(*self, Self::F32(_))
    }
    pub const fn is_u16(&self) -> bool {
        matches!(*self, Self::U16(_))
    }
    pub fn is_f32_and(self, f: impl FnOnce(Option<FormatterOutFloat>) -> bool) -> bool {
        match self {
            Self::U16(_) => false,
            Self::F32(x) => f(x),
        }
    }
    pub fn is_u16_and(self, f: impl FnOnce(Option<FormatterOut16>) -> bool) -> bool {
        match self {
            Self::U16(x) => f(x),
            Self::F32(_) => false,
        }
    }
}

pub(crate) fn default_input_formatter_factory(_type: u32, _flags: u32) -> FormatterIn {
    todo!()
}

pub(crate) fn default_output_formatter_factory(_type: u32, _flags: u32) -> FormatterOut {
    todo!()
}

pub(crate) static DEFAULT_FORMATTER_FACTORIES: (
    &'static FormatterInFactory,
    &'static FormatterOutFactory,
) = (
    &(default_input_formatter_factory as fn(u32, u32) -> FormatterIn),
    &(default_output_formatter_factory as fn(u32, u32) -> FormatterOut),
);
