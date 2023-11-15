use std::io::Result;

use crate::state::Context;

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
