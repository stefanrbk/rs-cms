use std::{io::Result, sync::Arc};

use crate::state::Context;

use super::IoHandler;

pub struct FileNull {
    used_space: usize,
    context_id: Context,
    pointer: usize,
    physical_file: String,
}

impl FileNull {
    fn new(context_id: &Context) -> Self {
        FileNull {
            pointer: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: String::new(),
        }
    }

    pub fn open(context_id: &Context) -> Arc<dyn IoHandler> {
        Arc::new(FileNull::new(&context_id))
    }
}
impl IoHandler for FileNull {
    fn context_id(&self) -> &Context {
        &self.context_id
    }

    fn used_space(&self) -> usize {
        self.used_space
    }

    fn reported_size(&self) -> usize {
        0
    }

    fn physical_file(&self) -> &str {
        &self.physical_file
    }

    fn read(&mut self, _buffer: &mut [u8], size: usize, count: usize) -> Result<usize> {
        let len = size * count;
        self.pointer += len;
        Ok(count)
    }

    fn seek(&mut self, offset: usize) -> Result<()> {
        self.pointer = offset;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.pointer)
    }

    fn write(&mut self, size: usize, _buffer: &[u8]) -> Result<()> {
        self.pointer += size;
        if self.pointer > self.used_space {
            self.used_space = self.pointer;
        }

        Ok(())
    }
}
