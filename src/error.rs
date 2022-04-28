use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Error type for nuclear, doubles as a wrapper for other libraries' Error types
pub enum Error {
    #[error("System error: {0}")]
    IOError(io::Error),
    #[error(
        "Expected file {file}{} to have magic {expected}, got {got}",
        match ftype {
            Some(c) => format!(" ({})", c),
            None => String::new(),
        }
    )]
    WrongFileKind {
        file: String,
        ftype: Option<String>,
        expected: String,
        got: String,
    },
    #[error("File {file} doesn't have section {s_name}, which is essential for it to work")]
    MissingRequiredSection { file: String, s_name: String },
    #[error("File {file} was given section {s_name}, which it doesn't recognize")]
    UnknownSection { file: String, s_name: String },
    #[error("Data in file {file} is invalid")]
    MalformedData { file: String },
    #[error("PNG format error - {0}")]
    PngFormatError(String),
    #[error("Something went wrong with the PNG library: {0}")]
    PngError(png::ParameterError),
    #[error("PNG format error - image data exceeded limits of image")]
    PngLimitError,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        panic!("This should never happen!")
    }
}

impl From<png::EncodingError> for Error {
    fn from(error: png::EncodingError) -> Self {
        match error {
            png::EncodingError::IoError(c) => Self::IOError(c),
            png::EncodingError::Format(c) => Self::PngFormatError(c.to_string()),
            png::EncodingError::Parameter(c) => Self::PngError(c),
            png::EncodingError::LimitsExceeded => Self::PngLimitError,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
