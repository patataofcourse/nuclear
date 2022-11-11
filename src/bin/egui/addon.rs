use crate::message;
use nuclear::error::Result;
pub trait NuclearResult<T> {
    fn manage(self) -> T
    where
        Self: Sized;
}

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
