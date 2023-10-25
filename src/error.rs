use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Error type for nuclear, doubles as a wrapper for other libraries' Error types
pub enum Error {
    //
    // Generic errors
    //
    /// Wrong magic for file
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

    /// Incorrect BOM
    #[error("File {file} has invalid Byte Order Mark")]
    InvalidBOM { file: String },

    /// File is missing a section essential for its completeness
    #[error("File {file} doesn't have section {s_name}, which is essential for it to work")]
    MissingRequiredSection { file: String, s_name: String },

    /// File has a section that the program doesn't recognize
    #[error("File {file} was given section {s_name}, which it doesn't recognize")]
    UnknownSection { file: String, s_name: String },

    /// Binary (Nintendo) file has invalid data
    #[error("Data in file {file} is invalid")]
    MalformedData { file: String },

    /// Error when loading project files
    #[error("Error when reading {0}: {1}")]
    FileFormatWrong(PathBuf, String),

    //
    // Wrappers
    //
    /// Wrapper for custom errors through String (good for one-off errors)
    #[error("{0}")]
    Generic(String),

    /// Wrapper for [std::io::Error]
    #[error("System error: {0}")]
    IOError(io::Error),

    /// Wrapper for [png::EncodingError::Format]
    #[error("PNG format error - {0}")]
    PngFormatError(String),

    /// Wrapper for [png::ParameterError]
    #[error("Something went wrong with the PNG library: {0}")]
    PngError(png::ParameterError),

    /// Wrapper for [png::EncodingError::LimitsExceeded]
    #[error("PNG format error - image data exceeded limits of image")]
    PngLimitError,

    /// Wrapper for [serde_json::Error]
    #[error("Saving or loading JSON file failed: {0}")]
    SerdeError(serde_json::Error),

    /// Error with the GUI library
    #[cfg(feature = "gui")]
    #[error("GUI library error: {0}")]
    GUIError(eframe::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
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

impl From<png::DecodingError> for Error {
    fn from(error: png::DecodingError) -> Self {
        match error {
            png::DecodingError::IoError(c) => Self::IOError(c),
            png::DecodingError::Format(c) => Self::PngFormatError(c.to_string()),
            png::DecodingError::Parameter(c) => Self::PngError(c),
            png::DecodingError::LimitsExceeded => Self::PngLimitError,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

#[cfg(feature = "gui")]
impl From<eframe::Error> for Error {
    fn from(error: eframe::Error) -> Self {
        Self::GUIError(error)
    }
}

/// Wrapper for [std::result::Result] using [enum@Error] as E
pub type Result<T> = std::result::Result<T, Error>;
