use std::fs::remove_file;
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::os::unix::fs::FileTypeExt; // for checking if a file is a FIFO
use std::path::Path;

extern crate libc;
use std::ffi::CString;

use crate::errors::{PipeError, StdStreamError};

pub fn read_pipe(pipe: &str) -> Result<String, PipeError> {
    match std::fs::OpenOptions::new().read(true).open(pipe) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(_) => Ok(line),
                Err(_) => Err(PipeError::FailedToReadError),
            }
        }
        Err(_) => Err(PipeError::PipeNotFoundError),
    }
}

pub fn write_pipe(pipe: &str, data: &str) -> Result<(), PipeError> {
    match std::fs::OpenOptions::new().write(true).open(pipe) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            match writer.write_all(data.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(PipeError::FailedToWriteError),
            }
        }
        Err(_) => Err(PipeError::PipeNotFoundError),
    }
}

fn fifo_pipe_available(pipe: &str) -> Result<bool, PipeError> {
    let path = Path::new(pipe);
    if path.exists() {
        match path.metadata() {
            Ok(metadata) => {
                if metadata.file_type().is_fifo() {
                    return Ok(true);
                } else {
                    return Err(PipeError::NotFIFOError)
                }
            }
            Err(_) => return Err(PipeError::FailedToGetMetadataError),
        }
    }
    Ok(false)
}

pub fn create_pipe(name: &str) -> Result<(), PipeError> {
    let fifo_available = match fifo_pipe_available(name) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };
    if fifo_available {
        return Ok(());
    }
    let c_name = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return Err(PipeError::InvalidNameError),
    };

    let mode = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH | libc::S_IWOTH;
    let res = unsafe { libc::mkfifo(c_name.as_ptr(), mode) };
    if res == 0 {
        Ok(())
    } else {
        let errno = unsafe { *libc::__errno_location() };
        if errno == libc::EEXIST {
            Ok(())
        } else {
            Err(PipeError::FailedToCreateError)
        }
    }
}

pub fn remove_pipe(name: &str) -> Result<(), PipeError> {
    match remove_file(name) {
        Ok(_) => Ok(()),
        Err(_) => Err(PipeError::FailedToRemoveError),
    }
}

pub fn read_stdin() -> Result<String, StdStreamError> {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();
    match stdin_lock.read_line(&mut line) {
        Ok(_) => Ok(line),
        Err(_) => Err(StdStreamError::FailedToReadError),
    }
}

pub fn write_stdout(data: &str) -> Result<(), StdStreamError> {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    match stdout_lock.write_all(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(StdStreamError::FailedToWriteError),
    }
}