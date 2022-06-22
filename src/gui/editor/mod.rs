use std::fmt::Display;

pub mod nclr;
pub mod tab;

pub use self::tab::render_tab;

pub enum EditorType {
    Palette,
    Tileset,
    Tilemap,
    Frame,
    Animation,
}

impl Display for EditorType {
    //TODO: localization
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Palette => "Palette",
                Self::Tileset => "Tileset",
                Self::Tilemap => "Tilemap",
                Self::Frame => "Frames",
                Self::Animation => "Animations",
            }
        )
    }
}
