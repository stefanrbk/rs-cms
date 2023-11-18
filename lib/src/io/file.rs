use std::{
    fs::OpenOptions,
    io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write},
    ops::DerefMut,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use log::Level;

use crate::state::{Context, ErrorCode};

use super::IoHandler;

pub trait FileLike: Read + Write + Seek {}
impl<T> FileLike for T where T: Read + Write + Seek {}

pub struct File {
    pub(crate) used_space: usize,
    pub(crate) reported_size: usize,
    pub(crate) context_id: Context,
    pub(crate) file: Arc<Mutex<dyn FileLike>>,
    pub(crate) physical_file: String,
}

impl File {
    fn new(context_id: &Context, file: Arc<Mutex<dyn FileLike>>, path: &str) -> Self {
        File {
            reported_size: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: path.to_owned(),
            file,
        }
    }

    pub fn open_for_reading(
        context_id: &Context,
        path: &Path,
        access: OpenOptions,
    ) -> Result<Arc<Mutex<dyn IoHandler>>> {
        let mut file = match access.clone().read(true).open(path) {
            Ok(f) => f,
            Err(error) => {
                return err!(context_id, Error, File, "File open error: {}", error; io => error);
            }
        };

        let file_len = file_length(&mut file)?;

        let mut file = File::new(
            &context_id,
            Arc::new(Mutex::new(file)),
            match path.to_str() {
                Some(path) => path,
                None => return err!(io => InvalidInput, "Path is invalid"),
            },
        );
        file.reported_size = file_len;

        Ok(Arc::new(Mutex::new(file)))
    }

    pub fn open_for_writing(
        context_id: &Context,
        path: &Path,
        access: OpenOptions,
    ) -> Result<Arc<Mutex<dyn IoHandler>>> {
        let file = match access.clone().write(true).open(path) {
            Ok(f) => f,
            Err(error) => {
                return err!(
                    context_id,
                    Error,
                    File,
                    "File open error: {}",
                    error;
                    io =>
                    error
                )
            }
        };

        let file = File::new(
            &context_id,
            Arc::new(Mutex::new(file)),
            match path.to_str() {
                Some(path) => path,
                None => return err!(io => InvalidInput, "Path is invalid"),
            },
        );

        Ok(Arc::new(Mutex::new(file)))
    }

    pub fn open_stream(
        context_id: &Context,
        stream: &Arc<Mutex<dyn FileLike>>,
    ) -> Result<Arc<Mutex<dyn IoHandler>>> {
        let mut file = stream.lock();
        let file = match file.as_mut() {
            Err(_) => return Err(Error::new(ErrorKind::Other, "Mutex error")),
            Ok(file) => file,
        };
        let file_size = file_length(file.deref_mut())?;

        let mut file = File::new(&context_id, stream.clone(), "");
        file.reported_size = file_size;

        Ok(Arc::new(Mutex::new(file)))
    }
}
impl IoHandler for File {
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
        let n_read = match self
            .file
            .lock()
            .unwrap()
            .read(&mut buffer[..(size * count)])
        {
            Ok(len) => len / size,
            Err(error) => {
                return err!(
                    self.context_id,
                    Error,
                    File,
                    "A read error occured: {}",
                    error;
                    io =>
                    error
                )
            }
        } / size;

        if n_read != count {
            return err!(
                self.context_id,
                Error,
                File,
                "Read error. Got {} bytes, block should be {} bytes",
                n_read * size,
                count * size;
                io =>
                UnexpectedEof,
                "Didn't read enough bytes"
            );
        }

        Ok(n_read)
    }

    fn seek(&mut self, offset: usize) -> Result<()> {
        self.file
            .lock()
            .unwrap()
            .seek(SeekFrom::Start(offset as u64))?;
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        match self.file.lock().unwrap().seek(SeekFrom::Current(0)) {
            Ok(pos) => Ok(pos as usize),
            Err(error) => {
                return err!(
                    self.context_id,
                    Error,
                    Seek,
                    "Tell error; probably corrupted profile";
                    io =>
                    error
                )
            }
        }
    }

    fn write(&mut self, size: usize, buffer: &[u8]) -> Result<()> {
        if size == 0 {
            return Ok(());
        } // We allow to write 0 bytes, but nothing happens

        self.used_space += size;

        match self.file.lock().unwrap().write_all(&buffer[..size]) {
            Ok(_) => Ok(()),
            Err(error) => err!(
                self.context_id,
                Error,
                Write,
                "Write error occured: {}",
                error;
                io =>
                error
            ),
        }
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

fn file_length(file: &mut dyn FileLike) -> Result<usize> {
    let p = file.seek(SeekFrom::Current(0))?;

    let n = file.seek(SeekFrom::End(0))?;

    file.seek(SeekFrom::Start(p))?;

    Ok(n as usize)
}
