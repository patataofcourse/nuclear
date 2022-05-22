use crate::{error::Result, img::{NCLR, NCGR, NSCR}};
use std::{collections::HashMap, path::PathBuf, fs::{self, File}, io::Write};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct NCLRWrapper {
    pub name: String,
    pub palettes: HashMap<u16, PathBuf>,
    pub is_8_bit: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NCGRWrapper {
    pub name: String,
    pub tiles: PathBuf,
    pub has_cpos: bool,
    pub is_8_bit: bool,
    pub ncbr_ff: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NSCRWrapper {
    pub name: String,
    pub map: PathBuf,
    pub width: u16,
    pub height: u16,
}

#[derive(Serialize, Deserialize)]
pub struct NuclearProject {
    pub name: String,
    pub author: String,
    pub palette_sets: Vec<NCLRWrapper>,
    pub tilesets: Vec<NCGRWrapper>,
    pub tilemaps: Vec<NSCRWrapper>,
    #[serde(skip, default)]
    path: PathBuf,
}

impl NuclearProject {
    pub fn new(name: &str, author: &str, path: PathBuf) -> Result<Self> {
        fs::create_dir_all(&path)?;

        let out = Self {
            name: name.to_string(),
            author: author.to_string(),
            palette_sets: vec![],
            tilesets: vec![],
            tilemaps: vec![],
            path: path,
        };

        out.write_meta()?;
        Ok(out)
    }

    fn write_meta(&self) -> Result<()> {
        let mut file = File::create(Self::proj_file_path(&self.path))?;
        write!(file, "{}", serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }

    fn proj_file_path(proj_path: &PathBuf) -> PathBuf {
        let mut path = proj_path.clone();
        path.extend(&PathBuf::from("nuclear_meta.json"));
        path
    }
}