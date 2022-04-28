use crate::{
    error::{Error, Result},
    ndsfile::NDSFile,
};
use bytestream::StreamReader;

pub type Tile = Vec<u8>;

#[derive(Debug, Clone)]
/// NCGR / NCBR tileset format
pub struct NCGR {
    pub is_8_bit: bool,
    pub tiles: Vec<Tile>,
    pub has_cpos: bool,
    pub ncbr_ff: bool,
    pub lineal_mode: bool,
}

impl NCGR {
    pub fn from_ndsfile(file: &NDSFile) -> Result<Self> {
        if file.magic != "RGCN" {
            Err(Error::WrongFileKind {
                file: file.fname.to_string(),
                ftype: Some("NCGR/NDS tile data".to_string().to_string()),
                expected: "RGCN".to_string(),
                got: file.magic.to_string(),
            })?
        }

        let mut is_8_bit = false;
        let mut has_cpos = false;
        let mut ncbr_ff = false;
        let mut lineal_mode = false;
        let mut tiles: Option<Vec<Tile>> = None;
        let o = file.byteorder;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.as_ref() {
                "RAHC" => {
                    let mut num_tiles = u16::read_from(&mut data, o)?;
                    u16::read_from(&mut data, o)?; // Tile size, always 0x20 in 4bit and 0x40 in 8bit
                    is_8_bit = u32::read_from(&mut data, o)? == 4;

                    u32::read_from(&mut data, o)?; // Padding
                    lineal_mode = u32::read_from(&mut data, o)? & 0xFF != 0;
                    let tile_data_size = u32::read_from(&mut data, o)?;
                    u32::read_from(&mut data, o)?; // Unknown, always 0x24

                    // For some reason some files do this - maybe only NCBR files?
                    if num_tiles == 0xFFFF {
                        ncbr_ff = true;
                        num_tiles = (tile_data_size / if is_8_bit { 0x40 } else { 0x20 }) as u16;
                    }

                    tiles = Some(vec![]);
                    let tilesvec = tiles.as_mut().unwrap();
                    if is_8_bit {
                        for _ in 0..num_tiles {
                            let mut tile = vec![];
                            for _ in 0..0x40 {
                                tile.push(u8::read_from(&mut data, o)?);
                            }
                            tilesvec.push(tile);
                        }
                    } else {
                        for _ in 0..num_tiles {
                            let mut tile = vec![];
                            for _ in 0..0x20 {
                                let eightbit = u8::read_from(&mut data, o)?;
                                tile.push(eightbit & 0xF);
                                tile.push(eightbit >> 4);
                            }
                            tilesvec.push(tile);
                        }
                    }
                }
                "SOPC" => has_cpos = true, //This section contains nothing of interest
                c => Err(Error::UnknownSection {
                    file: file.fname.clone(),
                    s_name: c.to_string(),
                })?,
            }
        }

        if let Some(c) = tiles {
            Ok(Self {
                tiles: c,
                is_8_bit,
                has_cpos,
                ncbr_ff,
                lineal_mode,
            })
        } else {
            Err(Error::MissingRequiredSection {
                file: file.fname.clone(),
                s_name: "CHAR".to_string(),
            })?
        }
    }
}

/*
NCGR
    CHAR
        0x00      : Common section header
        0x08 - u16: Tile count
        0x0A - u16: Tile size (ALWAYS 0x20)
        0x0C - u32: Color bit depth (3 is 4 bit, 4 is 8 bit)
        0x10 - u64: Padding? (0x00)
        0x18 - u32: Tile data size (tile count * tile size)
        0x1C - u32: Unknown - always 0x24
        0x20      : Start of tile data
    CPOS (optional, probably???)
        0x00      : Common section header
        0x08 - u32: Padding (0x00)
        0x0C - u16: Tile size (ALWAYS 0x20)
        0x1E - u16: Tile count
*/
