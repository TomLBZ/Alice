use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

struct PipeNotFoundError;
struct FailedToReadError;
struct FailedToWriteError;
struct FailedToGetMetadataError;
struct NotFIFOError;
struct InvalidNameError;
struct FailedToCreateError;
struct FailedToRemoveError;

#[derive(Debug, Clone)]
pub enum PipeError {
    PipeNotFoundError,
    FailedToReadError,
    FailedToWriteError,
    FailedToGetMetadataError,
    NotFIFOError,
    InvalidNameError,
    FailedToCreateError,
    FailedToRemoveError,
}

impl Error for PipeError {}

impl Display for PipeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::PipeNotFoundError => write!(f, "Pipe not found."),
            Self::FailedToReadError => write!(f, "Failed to read from pipe."),
            Self::FailedToWriteError => write!(f, "Failed to write to pipe."),
            Self::FailedToGetMetadataError => write!(f, "Failed to get metadata for pipe."),
            Self::NotFIFOError => write!(f, "Pipe is not a FIFO."),
            Self::InvalidNameError => write!(f, "Invalid pipe name."),
            Self::FailedToCreateError => write!(f, "Failed to create pipe."),
            Self::FailedToRemoveError => write!(f, "Failed to remove pipe."),
        }
    }
}

impl From<PipeNotFoundError> for PipeError {
    fn from(_: PipeNotFoundError) -> Self {
        Self::PipeNotFoundError
    }
}

impl From<FailedToReadError> for PipeError {
    fn from(_: FailedToReadError) -> Self {
        Self::FailedToReadError
    }
}

impl From<FailedToWriteError> for PipeError {
    fn from(_: FailedToWriteError) -> Self {
        Self::FailedToWriteError
    }
}

impl From<FailedToGetMetadataError> for PipeError {
    fn from(_: FailedToGetMetadataError) -> Self {
        Self::FailedToGetMetadataError
    }
}

impl From<NotFIFOError> for PipeError {
    fn from(_: NotFIFOError) -> Self {
        Self::NotFIFOError
    }
}

impl From<InvalidNameError> for PipeError {
    fn from(_: InvalidNameError) -> Self {
        Self::InvalidNameError
    }
}

impl From<FailedToCreateError> for PipeError {
    fn from(_: FailedToCreateError) -> Self {
        Self::FailedToCreateError
    }
}

impl From<FailedToRemoveError> for PipeError {
    fn from(_: FailedToRemoveError) -> Self {
        Self::FailedToRemoveError
    }
}

#[derive (Debug, Clone)]
pub enum StdStreamError {
    FailedToReadError,
    FailedToWriteError,
}
impl Error for StdStreamError {}

impl Display for StdStreamError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::FailedToReadError => write!(f, "Failed to read from stdin."),
            Self::FailedToWriteError => write!(f, "Failed to write to stdout."),
        }
    }
}

impl From<FailedToReadError> for StdStreamError {
    fn from(_: FailedToReadError) -> Self {
        Self::FailedToReadError
    }
}

impl From<FailedToWriteError> for StdStreamError {
    fn from(_: FailedToWriteError) -> Self {
        Self::FailedToWriteError
    }
}