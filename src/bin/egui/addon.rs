use std::error::Error;

use crate::message;
pub trait NuclearResult<T, E: Error> {
    fn manage(self) -> T
    where
        nuclear::error::Error: From<E>,
        Self: Sized;

    fn manage_explicit(self) -> T
    where
        Self: Sized;
}

impl<T, E: Error> NuclearResult<T, E> for Result<T, E> {
    fn manage(self) -> T {
        match self {
            Ok(c) => c,
            Err(e) => {
                message::error("Error happened!", &format!("Details:\n\n{}", e));
                std::process::exit(1)
            }
        }
    }

    fn manage_explicit(self) -> T {
        match self {
            Ok(c) => c,
            Err(e) => {
                message::error("Error happened!", &format!("Details:\n\n{}", e));
                std::process::exit(1)
            }
        }
    }
}
