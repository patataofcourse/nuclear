use crate::{
    error::{Error, Result},
    img::Tile,
    ndsfile::NDSFile,
};
use bytestream::StreamReader;

#[derive(Debug, Clone)]
/// NSCR (Nintendo SCreen Resource) tile image format
pub struct NSCR {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<TileRef>,
}

#[derive(Debug, Clone)]
/// Reference to a tile - NTFS (Nintendo Tile Format Screen) format
pub struct TileRef {
    pub tile: u16,
    pub flip_x: bool,
    pub flip_y: bool,
    pub palette: u8,
}

#[derive(Debug, Clone)]
/// A variant of [nuclear::img::NCGR] with horizontal mode forced, for NSCR referencing use
pub struct TilesForNSCR {
    /// Tile data
    pub tiles: Vec<Tile>,
    /// Indicates whether the file uses 8-bit color (true) or 4-bit color (false)
    pub is_8_bit: bool,
}

impl NSCR {
    pub fn from_ndsfile(file: &NDSFile) -> Result<Self> {
        if file.magic != "RCSN" {
            Err(Error::WrongFileKind {
                file: file.fname.to_string(),
                ftype: Some("NSCR/NDS image data".to_string().to_string()),
                expected: "RSCN".to_string(),
                got: file.magic.to_string(),
            })?
        }
        let o = file.byteorder;

        let mut width = 0;
        let mut height = 0;
        let mut tiles: Option<Vec<TileRef>> = None;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.as_ref() {
                "NRCS" => {
                    width = u16::read_from(&mut data, o)?;
                    height = u16::read_from(&mut data, o)?;
                    u32::read_from(&mut data, o)?; // padding?
                    let data_size = u32::read_from(&mut data, o)?;

                    let mut tile_vec = vec![];
                    for _ in 0..data_size / 2 {
                        let int = u16::read_from(&mut data, o)?;
                        let tile = int & 0x3FF;
                        let flip_x = int & 0x400 != 0;
                        let flip_y = int & 0x800 != 0;
                        let palette = (int >> 12) as u8;
                        tile_vec.push(TileRef {
                            tile,
                            flip_x,
                            flip_y,
                            palette,
                        })
                    }
                    tiles = Some(tile_vec);
                }
                c => Err(Error::UnknownSection {
                    file: file.fname.clone(),
                    s_name: c.to_string(),
                })?,
            }
        }

        if let Some(c) = tiles {
            Ok(Self {
                width,
                height,
                tiles: c,
            })
        } else {
            Err(Error::MissingRequiredSection {
                file: file.fname.clone(),
                s_name: "SCRN".to_string(),
            })?
        }
    }
}
