use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("System error: {0}")]
    IOError(io::Error),
    #[error(
        "Expected file {file}{} to have magic {expected}, got {got}",
        match ftype {
            Some(c) => &format!(" ({})", c),
            None => "",
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
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
