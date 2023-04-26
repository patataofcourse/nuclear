#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatType {
    Nintendo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    Palette,
    Tileset,
    Tilemap,
    Frames,
    Animation,
}

impl FormatType {
    pub fn filters(&self) -> Option<(&[&str], &str)> {
        match self {
            FormatType::Nintendo => Some((
                &[
                    "*.nclr", "*.ncgr", "*.ncbr", "*.nscr", "*.NCLR", "*.NCGR", "*.NCBR", "*.NSCR",
                ],
                ("Nintendo DS image files"),
            )),
        }
    }
}
