use std::fmt::Display;

pub mod pal;
pub mod tab;

#[derive(Clone, Copy)]
pub enum EditorType {
    Palette,
    Tileset,
    Tilemap,
    Frame,
    Animation,
}
#[derive(Clone, Debug)]
pub enum Editor {
    Palette(pal::PaletteEditor),
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
