use std::fs::remove_file;
use std::os::unix::fs::FileTypeExt; // for checking if a file is a FIFO
use std::path::Path;

extern crate libc;
use std::ffi::CString;

use crate::errors::PipeError;

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
