// stuff that adds onto types in nuclear itself

use super::message;
use crate::error::Result;

#[cfg(feature = "gui")]
pub trait NuclearResult<T> {
    fn manage(self) -> T
    where
        Self: Sized;
}

#[cfg(feature = "gui")]
impl<T> NuclearResult<T> for Result<T> {
    fn manage(self) -> T {
        match self {
            Ok(c) => c,
            Err(e) => {
                message::error("Error happened!", &format!("Details:\n\n{}", e));
                std::process::exit(1)
            }
        }
    }
}
