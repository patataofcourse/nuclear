use std::{collections::HashMap, path::PathBuf};

pub struct NCLRWrapper {
    pub name: String,
    pub palettes: HashMap<u16, PathBuf>,
    pub is_8_bit: bool,
}

pub struct NCGRWrapper {
    pub name: String,
    pub tiles: PathBuf,
    pub has_cpos: bool,
    pub is_8_bit: bool,
    pub ncbr_ff: bool,
}

pub struct NSCRWrapper {
    pub name: String,
    pub map: PathBuf,
    pub width: u16,
    pub height: u16,
}

pub struct NuclearProject {
    pub name: String,
    pub author: String,
    pub palette_sets: Vec<NCLRWrapper>,
    pub tilesets: Vec<NCGRWrapper>,
    pub tilemaps: Vec<NSCRWrapper>,
}

impl NuclearProject {
    pub fn new(name: &str, author: &str) -> Self {
        Self {
            name: name.to_string(),
            author: author.to_string(),
            palette_sets: vec![],
            tilesets: vec![],
            tilemaps: vec![],
        }
    }
}
