use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Error type for nuclear, doubles as a wrapper for other libraries' Error types
pub enum Error {
    #[error("System error: {0}")]
    /// Wrapper for [std::io::Error]
    IOError(io::Error),
    #[error(
        "Expected file {file}{} to have magic {expected}, got {got}",
        match ftype {
            Some(c) => format!(" ({})", c),
            None => String::new(),
        }
    )]
    /// Wrong magic for file
    WrongFileKind {
        file: String,
        ftype: Option<String>,
        expected: String,
        got: String,
    },
    #[error("File {file} doesn't have section {s_name}, which is essential for it to work")]
    /// File is missing a section essential for its completeness
    MissingRequiredSection { file: String, s_name: String },
    #[error("File {file} was given section {s_name}, which it doesn't recognize")]
    /// File has a section that the program doesn't recognize
    UnknownSection { file: String, s_name: String },
    #[error("Data in file {file} is invalid")]
    /// File has invalid data
    MalformedData { file: String },
    #[error("PNG format error - {0}")]
    /// Wrapper for [png::EncodingError::Format]
    PngFormatError(String),
    #[error("Something went wrong with the PNG library: {0}")]
    /// Wrapper for [png::ParameterError]
    PngError(png::ParameterError),
    #[error("PNG format error - image data exceeded limits of image")]
    /// Wrapper for [png::EncodingError::LimitsExceeded]
    PngLimitError,
    #[error("Saving or loading JSON file failed: {0}")]
    /// JSON Serialization error
    SerdeError(serde_json::Error),
    #[error("Error when reading {0}: {1}")]
    FileFormatWrong(PathBuf, String),
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

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

/// Wrapper for [std::result::Result] using [enum@Error] as E
pub type Result<T> = std::result::Result<T, Error>;
