pub mod error;
pub mod img;
pub mod ndsfile;
pub mod proj;

#[cfg(feature = "gui")]
pub(crate) mod gui;

#[cfg(feature = "gui")]
pub use gui::{panic_hook, NuclearApp};
