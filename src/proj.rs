use crate::{
    error::Result,
    img::{NCGR, NCLR, NSCR},
};
use bytestream::{ByteOrder, StreamWriter};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct NCLRWrapper {
    pub name: String,
    pub palettes: HashMap<u16, PathBuf>,
    pub is_8_bit: bool,
    #[serde(skip, default)]
    pub bin: HashMap<u16, Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct NCGRWrapper {
    pub name: String,
    pub tiles: PathBuf,
    pub has_cpos: bool,
    pub is_8_bit: bool,
    pub ncbr_ff: bool,
    #[serde(skip, default)]
    pub bin: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct NSCRWrapper {
    pub name: String,
    pub map: PathBuf,
    pub width: u16,
    pub height: u16,
    #[serde(skip, default)]
    pub bin: Vec<u8>,
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
    /// Creates a new nuclear project on the specified path
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

    fn write_file(&self, path: &PathBuf, contents: &[u8]) -> Result<()> {
        let mut file = File::create(path)?;
        file.write(contents)?;
        Ok(())
    }

    fn proj_file_path(proj_path: &PathBuf) -> PathBuf {
        let mut path = proj_path.clone();
        path.extend(&PathBuf::from("nuclear_meta.json"));
        path
    }

    /// Saves the project
    pub fn save(&self) -> Result<()> {
        self.write_meta()?;
        Ok(())
    }

    pub fn add_nclr(mut self, name: &str, nclr: &NCLR) -> Result<()> {
        let mut path = self.path.clone();
        path.extend(&PathBuf::from(format!("pal/{}", name)));
        fs::create_dir_all(&path)?;

        let mut files = HashMap::new();
        let mut binaries = HashMap::new();

        for (id, palette) in &nclr.palettes {
            let mut path = path.clone();
            path.extend(&PathBuf::from(format!("pal{:X}.bin", id)));
            let mut bin = vec![];
            for color in palette {
                color.write_to(&mut bin, ByteOrder::LittleEndian)?;
            }

            self.write_file(&path, &bin)?;

            files.insert(*id, PathBuf::from(format!("pal{:X}.bin", id)));
            binaries.insert(*id, bin);
        }

        self.palette_sets.push(NCLRWrapper {
            name: name.to_string(),
            is_8_bit: nclr.is_8_bit,
            palettes: files,
            bin: binaries,
        });
        self.write_meta()?;
        Ok(())
    }
}
