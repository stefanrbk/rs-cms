use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Signature(pub u32);

impl Signature {}

impl From<Signature> for u32 {
    fn from(value: Signature) -> Self {
        value.0
    }
}

impl From<u32> for Signature {
    fn from(value: u32) -> Self {
        Signature(value)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = [char::default(); 4];

        buf[0] = (self.0 & 0x7f) as u8 as char;
        buf[1] = ((self.0 >> 8) & 0x7f) as u8 as char;
        buf[2] = ((self.0 >> 16) & 0x7f) as u8 as char;
        buf[3] = ((self.0 >> 24) & 0x7f) as u8 as char;

        write!(f, "{}{}{}{}", buf[0], buf[1], buf[2], buf[3])
    }
}
