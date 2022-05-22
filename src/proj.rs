use crate::{
    error::{Error, Result},
    img::{ncgr::NCGRTiles, ColorBGR555, NCGR, NCLR, NSCR},
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct NCLRWrapper {
    pub palettes: HashMap<u16, PathBuf>,
    pub is_8_bit: bool,
    #[serde(skip, default)]
    pub bin: HashMap<u16, Vec<u8>>, // to be loaded at project load
}

#[derive(Serialize, Deserialize)]
pub struct NCGRWrapper {
    pub tiles: PathBuf,
    pub has_cpos: bool,
    pub is_8_bit: bool,
    pub ncbr_ff: bool,
    pub lineal_mode: bool,
    #[serde(skip, default)]
    pub bin: Vec<u8>, // to be loaded at project load
}

#[derive(Serialize, Deserialize)]
pub struct NSCRWrapper {
    pub map: PathBuf,
    pub width: u16,
    pub height: u16,
    #[serde(skip, default)]
    pub bin: Vec<u8>, // to be loaded at project load
}

#[derive(Serialize, Deserialize)]
pub struct NuclearProject {
    pub name: String,
    pub author: String,
    pub palette_sets: HashMap<String, NCLRWrapper>,
    pub tilesets: HashMap<String, NCGRWrapper>,
    pub tilemaps: HashMap<String, NSCRWrapper>,
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
            palette_sets: HashMap::new(),
            tilesets: HashMap::new(),
            tilemaps: HashMap::new(),
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

    fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>> {
        let mut path_ = self.path.clone();
        path_.extend(path);
        let mut file = File::open(path)?;
        let mut buffer = vec![];
        file.read(&mut buffer)?;
        Ok(buffer)
    }

    fn proj_file_path(proj_path: &PathBuf) -> PathBuf {
        let mut path = proj_path.clone();
        path.extend(&PathBuf::from("nuclear_meta.json"));
        path
    }

    /// Saves the entire project
    pub fn save(&self) -> Result<()> {
        self.write_meta()?;
        Ok(())
    }

    /// Adds a NCLR file to the project. If it already exists, it replaces the previous version.
    /// Will reset the palette files to their original positions!!
    pub fn insert_nclr(&mut self, name: &str, nclr: &NCLR) -> Result<()> {
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

        self.palette_sets.insert(
            name.to_string(),
            NCLRWrapper {
                is_8_bit: nclr.is_8_bit,
                palettes: files,
                bin: binaries,
            },
        );
        self.write_meta()?;
        Ok(())
    }

    /// Gets the specified NCLR file from the project
    pub fn get_nclr(&self, name: &str) -> Result<Option<NCLR>> {
        let wrapper = match self.palette_sets.get(name) {
            Some(c) => c,
            None => return Ok(None),
        };
        let mut palettes = HashMap::new();
        let mut color_amt = 0;
        for (id, pal) in &wrapper.bin {
            let mut pal: &[u8] = &pal;
            let mut colors = vec![];
            while pal.len() != 0 {
                colors.push(ColorBGR555::read_from(&mut pal, ByteOrder::LittleEndian)?);
            }

            if color_amt == 0 {
                color_amt = colors.len();
            } else if colors.len() != color_amt {
                Err(Error::FileFormatWrong(
                    wrapper.palettes.get(&id).unwrap().to_path_buf(),
                    format!(
                        "All palletes must have same number of colors (found {} and {})",
                        colors.len(),
                        color_amt
                    ),
                ))?;
            }

            palettes.insert(*id, colors);
        }
        Ok(Some(NCLR {
            palettes,
            is_8_bit: wrapper.is_8_bit,
            color_amt: color_amt as u32,
        }))
    }

    /// Adds a NCGR file to the project. If it already exists, it replaces the previous version.
    /// Will reset the tile file to its original position!!
    pub fn insert_ncgr(&mut self, name: &str, ncgr: &NCGR) -> Result<()> {
        let mut path = self.path.clone();
        path.extend(&PathBuf::from("img"));
        fs::create_dir_all(&path)?;
        path.extend(&PathBuf::from(format!("tile_{}.bin", name)));

        let binary: Vec<u8>;
        let lineal_mode: bool;
        match &ncgr.tiles {
            NCGRTiles::Horizontal(c) => {
                lineal_mode = false;
                let mut bin: Vec<u8> = vec![];
                for tile in c {
                    bin.extend(tile);
                }
                binary = bin;
            }
            NCGRTiles::Lineal(c) => {
                lineal_mode = true;
                binary = c.clone();
            }
        }

        let mut file = File::create(&path)?;
        file.write(&binary)?;

        self.tilesets.insert(
            name.to_string(),
            NCGRWrapper {
                is_8_bit: ncgr.is_8_bit,
                ncbr_ff: ncgr.ncbr_ff,
                lineal_mode,
                has_cpos: ncgr.has_cpos,
                tiles: path,
                bin: binary,
            },
        );
        self.write_meta()?;
        Ok(())
    }

    /// Gets the specified NCGR file from the project
    pub fn get_ncgr(&self, name: &str) -> Result<Option<NCGR>> {
        let wrapper = match self.tilesets.get(name) {
            Some(c) => c,
            None => return Ok(None),
        };

        let tiles = NCGRTiles::from_tile_data(
            &mut wrapper.bin.as_ref(),
            wrapper.bin.len() / if wrapper.is_8_bit { 0x40 } else { 0x20 },
            wrapper.lineal_mode,
            true,
        );

        Ok(Some(NCGR {
            tiles,
            has_cpos: wrapper.has_cpos,
            is_8_bit: wrapper.is_8_bit,
            ncbr_ff: wrapper.ncbr_ff,
        }))
    }
}
