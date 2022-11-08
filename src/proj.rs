use crate::{
    error::{Error, Result},
    img::{
        ncgr::{NCGRTiles, Tile},
        nscr::TileRef,
        ColorBGR555, NCGR, NCLR, NSCR,
    },
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct NCLRWrapper {
    pub folder: PathBuf,
    pub palettes: BTreeMap<u16, PathBuf>,
    pub is_8_bit: bool,
    #[serde(skip, default)]
    pub bin: BTreeMap<u16, Vec<u8>>, // to be loaded at project load
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NCGRWrapper {
    pub tiles: PathBuf,
    pub has_cpos: bool,
    pub is_8_bit: bool,
    pub ncbr_ff: bool,
    pub lineal_mode: bool,
    #[serde(skip, default)]
    pub bin: Vec<u8>, // to be loaded at project load
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NSCRWrapper {
    pub map: PathBuf,
    pub width: u16,
    pub height: u16,
    #[serde(skip, default)]
    pub bin: Vec<u8>, // to be loaded at project load
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NuclearProject {
    pub name: String,
    pub author: String,
    pub description: String,
    pub palette_sets: HashMap<String, NCLRWrapper>,
    pub tilesets: HashMap<String, NCGRWrapper>,
    pub tilemaps: HashMap<String, NSCRWrapper>,
    //TODO: NCER, NANR
    #[serde(skip, default)]
    path: PathBuf,
}

impl NuclearProject {
    /// Creates a new nuclear project on the specified path
    pub fn new(name: &str, author: &str, description: &str, path: PathBuf) -> Result<Self> {
        fs::create_dir_all(&path)?;

        let out = Self {
            name: name.to_string(),
            author: author.to_string(),
            description: description.to_string(),
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

    fn read_file(own_path: &PathBuf, path: &PathBuf) -> Result<Vec<u8>> {
        let mut path_ = own_path.clone();
        path_.extend(path);
        let mut file = File::open(path_)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn proj_file_path(proj_path: &PathBuf) -> PathBuf {
        let mut path = proj_path.clone();
        path.extend(&PathBuf::from("nuclear_meta.json"));
        path
    }

    /// Saves the entire project
    pub fn save(&self) -> Result<()> {
        self.write_meta()
    }

    pub fn load_from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let meta_path = Self::proj_file_path(&path);
        let mut meta = File::open(meta_path)?;

        let mut json = String::new();
        meta.read_to_string(&mut json)?;

        let mut project: NuclearProject = serde_json::from_str(&json)?;
        project.path = path;

        let path: PathBuf = "pal".into();
        for pal in &mut project.palette_sets {
            let pal = pal.1;
            let mut path = path.clone();
            path.extend(&pal.folder);
            for palette in &pal.palettes {
                let mut path = path.clone();
                path.extend(palette.1);
                pal.bin
                    .insert(*palette.0, Self::read_file(&project.path, &path)?);
            }
        }

        let path: PathBuf = "img".into();
        for tiles in &mut project.tilesets {
            let tiles = tiles.1;
            let mut path = path.clone();
            path.extend(&tiles.tiles);
            tiles.bin = Self::read_file(&project.path, &path)?;
        }

        let path: PathBuf = "map".into();
        for map in &mut project.tilemaps {
            let map = map.1;
            let mut path = path.clone();
            path.extend(&map.map);
            map.bin = Self::read_file(&project.path, &path)?;
        }

        //TODO: NCER, NANR

        Ok(project)
    }

    /// Adds a NCLR file to the project. If it already exists, it replaces the previous version.
    /// Will reset the palette files to their original positions!!
    pub fn insert_nclr(&mut self, name: &str, nclr: &NCLR) -> Result<()> {
        let mut path = self.path.clone();
        path.extend(&PathBuf::from(format!("pal/{}", name)));
        fs::create_dir_all(&path)?;

        let mut files = BTreeMap::new();
        let mut binaries = BTreeMap::new();

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
                folder: name.into(),
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
        let mut palettes = BTreeMap::new();
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
        let fname = PathBuf::from(format!("tile_{}.bin", name));

        let mut path = self.path.clone();
        path.extend(&PathBuf::from("img"));
        fs::create_dir_all(&path)?;
        path.extend(&fname);

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
                tiles: fname,
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

        let tiles = if wrapper.lineal_mode {
            NCGRTiles::Lineal(wrapper.bin.clone())
        } else {
            let mut tiles: Vec<Tile> = vec![];
            let mut f: &[u8] = &wrapper.bin;
            for _ in 0..wrapper.bin.len() / 64 {
                let t: Vec<u8>;
                let mut tslice = [0u8; 64];
                f.read(&mut tslice)?;
                t = tslice.into();
                tiles.push(t);
            }
            NCGRTiles::Horizontal(tiles)
        };

        Ok(Some(NCGR {
            tiles,
            has_cpos: wrapper.has_cpos,
            is_8_bit: wrapper.is_8_bit,
            ncbr_ff: wrapper.ncbr_ff,
        }))
    }

    /// Adds a NSCR file to the project. If it already exists, it replaces the previous version.
    /// Will reset the tilemap file to its original position!!
    pub fn insert_nscr(&mut self, name: &str, nscr: &NSCR) -> Result<()> {
        let fname = PathBuf::from(format!("map_{}.bin", name));

        let mut path = self.path.clone();
        path.extend(&PathBuf::from("map"));
        fs::create_dir_all(&path)?;
        path.extend(&fname);

        let mut binary = vec![];
        for tile in &nscr.tiles {
            tile.tile.write_to(&mut binary, ByteOrder::LittleEndian)?;
            tile.flip_x.write_to(&mut binary, ByteOrder::LittleEndian)?;
            tile.flip_y.write_to(&mut binary, ByteOrder::LittleEndian)?;
            #[rustfmt::skip]
            tile.palette.write_to(&mut binary, ByteOrder::LittleEndian)?;
        }

        let mut file = File::create(&path)?;
        file.write(&binary)?;

        self.tilemaps.insert(
            name.to_string(),
            NSCRWrapper {
                map: fname,
                width: nscr.width,
                height: nscr.height,
                bin: binary,
            },
        );
        self.write_meta()?;
        Ok(())
    }

    /// Gets the specified NSCR file from the project
    pub fn get_nscr(&self, name: &str) -> Result<Option<NSCR>> {
        let wrapper = match self.tilemaps.get(name) {
            Some(c) => c,
            None => return Ok(None),
        };

        let mut tiles = vec![];
        let mut bin: &[u8] = &wrapper.bin;
        while bin.len() != 0 {
            tiles.push(TileRef {
                tile: u16::read_from(&mut bin, ByteOrder::LittleEndian)?,
                flip_x: bool::read_from(&mut bin, ByteOrder::LittleEndian)?,
                flip_y: bool::read_from(&mut bin, ByteOrder::LittleEndian)?,
                palette: u8::read_from(&mut bin, ByteOrder::LittleEndian)?,
            })
        }

        Ok(Some(NSCR {
            tiles,
            width: wrapper.width,
            height: wrapper.height,
        }))
    }
}
