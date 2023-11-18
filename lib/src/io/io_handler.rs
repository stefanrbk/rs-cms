use std::{io::Result, mem::size_of};

use crate::{
    align_long, f64_to_s15_fixed16_number, f64_to_u16_fixed16_number, s15_fixed16_number_to_f64,
    state::Context,
    types::{Signature, XYZ},
    u16_fixed16_number_to_f64, S15Fixed16Number,
};

pub trait IoHandler {
    fn context_id(&self) -> &Context;
    fn used_space(&self) -> usize;
    fn reported_size(&self) -> usize;
    fn physical_file(&self) -> &str;
    fn read(&mut self, buffer: &mut [u8], size: usize, count: usize) -> Result<usize>;
    fn seek(&mut self, offset: usize) -> Result<()>;
    fn close(&mut self) -> Result<()>;
    fn tell(&mut self) -> Result<usize>;
    fn write(&mut self, size: usize, buffer: &[u8]) -> Result<()>;
}

impl dyn IoHandler + '_ {
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut tmp = [0u8; size_of::<u8>()];
        if self.read(&mut tmp, size_of::<u8>(), 1)? != 1 {
            err!(
                io =>
                UnexpectedEof,
                "Read error in read_u8"
            )
        } else {
            Ok(tmp[0])
        }
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let mut tmp = [0u8; size_of::<u16>()];
        if self.read(&mut tmp, size_of::<u16>(), 1)? != 1 {
            return err!(
                io =>
                UnexpectedEof,
                "Read error in read_u16"
            );
        }

        Ok(u16::from_be_bytes(tmp))
    }

    pub fn read_u16_slice<'a>(&mut self, array: &'a mut [u16]) -> Result<&'a [u16]> {
        for i in 0..array.len() {
            array[i] = self.read_u16()?;
        }

        Ok(array)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut tmp = [0u8; size_of::<u32>()];
        if self.read(&mut tmp, size_of::<u32>(), 1)? != 1 {
            return err!(
                io =>
                UnexpectedEof,
                "Read error in read_u32"
            );
        }

        Ok(u32::from_be_bytes(tmp))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let mut tmp = [0u8; size_of::<f32>()];
        if self.read(&mut tmp, size_of::<f32>(), 1)? != 1 {
            return err!(
                io =>
                UnexpectedEof,
                "Read error in read_f32"
            );
        }

        let result = f32::from_bits(u32::from_be_bytes(tmp));
        if result > 1e20f32 || result < -1e20f32 || !(result.is_normal() || result == 0f32) {
            return err!(
                io =>
                Other,
                "Float values are out of bounds in read_f32"
            );
        }
        Ok(result)
    }

    pub fn read_signature(&mut self) -> Result<Signature> {
        Ok(Signature(self.read_u32()?))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let mut tmp = [0u8; size_of::<u64>()];
        if self.read(&mut tmp, size_of::<u64>(), 1)? != 1 {
            return err!(
                io =>
                UnexpectedEof,
                "Read error in read_u64"
            );
        }

        Ok(u64::from_be_bytes(tmp))
    }

    pub fn read_s15_fixed16_number(&mut self) -> Result<f64> {
        Ok(s15_fixed16_number_to_f64(
            self.read_u32()? as S15Fixed16Number
        ))
    }

    pub fn read_u16_fixed16_number(&mut self) -> Result<f64> {
        Ok(u16_fixed16_number_to_f64(self.read_u32()?))
    }

    pub fn read_xyz(&mut self) -> Result<XYZ> {
        Ok(XYZ {
            x: self.read_s15_fixed16_number()?,
            y: self.read_s15_fixed16_number()?,
            z: self.read_s15_fixed16_number()?,
        })
    }

    pub fn write_u8(&mut self, n: u8) -> Result<()> {
        let tmp = [n];
        Ok(self.write(size_of::<u8>(), &tmp)?)
    }

    pub fn write_u16(&mut self, n: u16) -> Result<()> {
        let tmp = n.to_be_bytes();
        Ok(self.write(size_of::<u16>(), &tmp)?)
    }

    pub fn write_u16_slice(&mut self, array: &[u16]) -> Result<()> {
        for n in array {
            self.write_u16(*n)?
        }

        Ok(())
    }

    pub fn write_u32(&mut self, n: u32) -> Result<()> {
        let tmp = n.to_be_bytes();
        Ok(self.write(size_of::<u32>(), &tmp)?)
    }

    pub fn write_f32(&mut self, n: f32) -> Result<()> {
        let tmp = n.to_bits().to_be_bytes();
        Ok(self.write(size_of::<f32>(), &tmp)?)
    }

    pub fn write_signature(&mut self, n: Signature) -> Result<()> {
        Ok(self.write_u32(n.0)?)
    }

    pub fn write_u64(&mut self, n: u64) -> Result<()> {
        let tmp = n.to_be_bytes();
        Ok(self.write(size_of::<u64>(), &tmp)?)
    }

    pub fn write_s15_fixed16_number(&mut self, n: f64) -> Result<()> {
        let n = f64_to_s15_fixed16_number(n) as u32;
        Ok(self.write_u32(n)?)
    }

    pub fn write_u16_fixed16_number(&mut self, n: f64) -> Result<()> {
        let n = f64_to_u16_fixed16_number(n);
        Ok(self.write_u32(n)?)
    }

    pub fn write_xyz(&mut self, xyz: XYZ) -> Result<()> {
        self.write_s15_fixed16_number(xyz.x)?;
        self.write_s15_fixed16_number(xyz.y)?;
        self.write_s15_fixed16_number(xyz.z)?;

        Ok(())
    }

    pub fn read_type_base(&mut self) -> Result<Signature> {
        let result = Signature(self.read_u32()?);
        _ = Signature(self.read_u32()?);

        Ok(result)
    }

    pub fn write_type_base(&mut self, sig: Signature) -> Result<()> {
        self.write_signature(sig)?;
        self.write_u32(0)?;

        Ok(())
    }

    pub fn read_alignment(&mut self) -> Result<()> {
        let mut buffer = [0u8; 4];

        let at = self.tell()?;
        let next_aligned = align_long(at);
        let bytes_to_next_aligned_pos = next_aligned - at;
        if bytes_to_next_aligned_pos == 0 {
            return Ok(());
        }

        if bytes_to_next_aligned_pos > 4 {
            return err!(io => Other,"Alignment issues in read_alignment");
        }

        if self.read(&mut buffer, bytes_to_next_aligned_pos, 1)? != 1 {
            err!(io => Other, "Read error in read_alignment")
        } else {
            Ok(())
        }
    }

    pub fn write_alignment(&mut self) -> Result<()> {
        let buffer = [0u8; 4];

        let at = self.tell()?;
        let next_aligned = align_long(at);
        let bytes_to_next_aligned_pos = next_aligned - at;
        if bytes_to_next_aligned_pos == 0 {
            return Ok(());
        }

        if bytes_to_next_aligned_pos > 4 {
            return err!(io => Other, "Alignment issues in write_alignment");
        }

        self.write(bytes_to_next_aligned_pos, &buffer)
    }
}
