pub mod nclr;

pub enum EditorType {
    Palette,
    Tileset,
    Tilemap,
    Frame,
    Animation,
}

impl ToString for EditorType {
    //TODO: localization
    fn to_string(&self) -> String {
        match self {
            Self::Palette => String::from("Palette"),
            Self::Tileset => String::from("Tileset"),
            Self::Tilemap => String::from("Tilemap"),
            Self::Frame => String::from("Frame"),
            Self::Animation => String::from("Animation"),
        }
    }
}
