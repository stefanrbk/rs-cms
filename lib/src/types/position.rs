#[repr(C)]
pub struct PositionNumber {
    offset: u32,
    size: u32,
}

impl PositionNumber {
    pub fn get_offset(&self) -> usize {
        self.offset as usize
    }

    pub fn get_size(&self) -> usize {
        self.size as usize
    }
    pub fn set_offset(&mut self, value: usize) {
        self.offset = value as u32
    }

    pub fn set_size(&mut self, value: usize) {
        self.size = value as u32
    }
}
