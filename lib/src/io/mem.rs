use std::{
    io::{ErrorKind, Result},
    sync::{Arc, Mutex},
};

use log::Level;

use crate::state::{Context, ErrorCode};

use super::IoHandler;

pub struct FileMem {
    pub(crate) used_space: usize,
    pub(crate) reported_size: usize,
    pub(crate) context_id: Context,
    pub(crate) block: Arc<Mutex<Box<[u8]>>>,
    pub(crate) pointer: usize,
    pub(crate) physical_file: String,
}

impl FileMem {
    fn new(context_id: &Context, block: &Arc<Mutex<Box<[u8]>>>) -> Self {
        FileMem {
            pointer: 0,
            reported_size: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: String::new(),
            block: block.clone(),
        }
    }

    pub fn open_for_reading(context_id: &Context, buffer: &[u8]) -> Arc<Mutex<dyn IoHandler>> {
        let mut block = vec![0u8; buffer.len()];
        block.copy_from_slice(&buffer);

        let mut mem = FileMem::new(&context_id, &Arc::new(Mutex::new(block.into_boxed_slice())));
        mem.reported_size = buffer.len();

        Arc::new(Mutex::new(mem))
    }
    pub fn open_for_writing(
        context_id: &Context,
        buffer: &Arc<Mutex<Box<[u8]>>>,
    ) -> Arc<Mutex<dyn IoHandler>> {
        let mem = FileMem::new(&context_id, &buffer);

        Arc::new(Mutex::new(mem))
    }
}
impl IoHandler for FileMem {
    fn context_id(&self) -> &Context {
        &self.context_id
    }

    fn used_space(&self) -> usize {
        self.used_space
    }

    fn reported_size(&self) -> usize {
        self.reported_size
    }

    fn physical_file(&self) -> &str {
        &self.physical_file
    }

    fn read(&mut self, buffer: &mut [u8], size: usize, count: usize) -> Result<usize> {
        let len = size * count;
        let block_size = self.block.lock().unwrap().len();

        if self.pointer + len > block_size {
            let len = block_size - self.pointer;
            self.context_id
                .signal_error(Level::Error, ErrorCode::Read, &format!(
                    "Read from memory error. Got {} bytes, block should be of {} bytes",
                    len,
                    count * size
                ));
            return Err(ErrorKind::UnexpectedEof.into());
        }

        let ptr = &self.block.lock().unwrap()[self.pointer..];
        buffer[..len].copy_from_slice(&ptr[..len]);
        self.pointer += len;

        Ok(count)
    }

    fn seek(&mut self, offset: usize) -> Result<()> {
        if offset > self.block.lock().unwrap().len() {
            self.context_id.signal_error(
                Level::Error,
                ErrorCode::Seek,
                "Too few data; probably corrupted profile",
            );
            return Err(ErrorKind::InvalidInput.into());
        }
        self.pointer = offset;
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        if self.block.lock().unwrap().len() == 0 {
            return Ok(0);
        }

        Ok(self.pointer)
    }

    fn write(&mut self, mut size: usize, buffer: &[u8]) -> Result<()> {
        let block_size = self.block.lock().unwrap().len();

        // Housekeeping
        if block_size == 0 {
            return Err(ErrorKind::WriteZero.into());
        }

        // Check for available space. Clip.
        if self.pointer + size > block_size {
            size = block_size - self.pointer;
        }

        // Write zero bytes is ok, but does nothing
        if size == 0 {
            return Ok(());
        }

        self.block.lock().unwrap()[self.pointer..][..size].copy_from_slice(&buffer[..size]);
        self.pointer += size;

        if self.pointer > self.used_space {
            self.used_space = self.pointer;
        }

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
