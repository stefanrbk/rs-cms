use crate::S15Fixed16Number;

#[repr(C)]
pub struct ResponseNumber {
    pub device_code: u16,
    pub measurement: S15Fixed16Number,
}
