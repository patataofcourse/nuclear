use crate::{
    error::{Error, Result},
    ndsfile::{NDSFile, Section},
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::{io::Write, ops::Range};

/// Represents an NDS tile
pub type Tile = Vec<u8>;

#[derive(Debug, Clone)]
/// NCGR (Nintendo Character Graphics Resource) / NCBR (Nintendo Character Basic Resource) tileset format
pub struct NCGR {
    /// Enum containing tile data
    pub tiles: NCGRTiles,
    /// Indicates whether the file uses 8-bit color (true) or 4-bit color (false)
    pub is_8_bit: bool,
    /// Indicates whether the file has a CPOS section
    pub has_cpos: bool,
    /// Indicates whether the file's tile amount was set to 0xFFFF - believed to happen only in NCBR files
    pub ncbr_ff: bool,
}

#[derive(Debug, Clone)]
/// Contains raw tile data (names extracted from Tinke)
pub enum NCGRTiles {
    /// Format in which gfx data isn't split into tiles per se
    ///
    /// TODO: explain format better
    Lineal(Vec<u8>),
    /// Format in which gfx data is split into 8x8 tiles
    Horizontal(Vec<Tile>),
}

impl NCGR {
    /// Creates an NCGR struct from the NDSFile given
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
        let mut num_tiles = 0;
        let mut tiles: Option<Vec<u8>> = None;
        let o = file.byteorder;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.as_ref() {
                "RAHC" => {
                    num_tiles = u16::read_from(&mut data, o)?;
                    let num_tiles_2 = u16::read_from(&mut data, o)?;
                    is_8_bit = u32::read_from(&mut data, o)? == 4;

                    u32::read_from(&mut data, o)?; // Padding
                    lineal_mode = (u32::read_from(&mut data, o)? & 0xFF) != 0;
                    let tile_data_size = u32::read_from(&mut data, o)?;
                    u32::read_from(&mut data, o)?; // Unknown, always 0x18

                    // For some reason some files do this - maybe only NCBR files?
                    if num_tiles == 0xFFFF {
                        ncbr_ff = true;
                        num_tiles = (tile_data_size / if is_8_bit { 0x40 } else { 0x20 }) as u16;
                    } else {
                        num_tiles *= num_tiles_2
                    }

                    tiles = Some(vec![]);
                    let tile_contents = tiles.as_mut().unwrap();
                    for _ in 0..num_tiles as usize * if is_8_bit { 0x40 } else { 0x20 } {
                        tile_contents.push(u8::read_from(&mut data, o)?)
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
                tiles: NCGRTiles::from_tile_data(
                    &mut c.as_ref(),
                    num_tiles as usize,
                    lineal_mode,
                    is_8_bit,
                ),
                is_8_bit,
                has_cpos,
                ncbr_ff,
            })
        } else {
            Err(Error::MissingRequiredSection {
                file: file.fname.clone(),
                s_name: "CHAR".to_string(),
            })?
        }
    }

    /// Creates an NDSFile from the NCGR struct gives
    pub fn to_ndsfile(&self, fname: String, o: ByteOrder) -> Result<NDSFile> {
        // CHAR section
        let ref mut char_buff: Vec<u8> = vec![];
        if self.ncbr_ff {
            (-1i32).write_to(char_buff, o)?;
        } else {
            (self.tiles.len(self.is_8_bit) as u16).write_to(char_buff, o)?;
            if self.is_8_bit { 0x40u16 } else { 0x20 }.write_to(char_buff, o)?;
        }
        if self.is_8_bit { 4u32 } else { 3u32 }.write_to(char_buff, o)?;
        0u32.write_to(char_buff, o)?;
        let tile_data_size;
        match &self.tiles {
            NCGRTiles::Horizontal(c) => {
                0u32.write_to(char_buff, o)?;
                tile_data_size = c.len() as u32 * if self.is_8_bit { 0x40 } else { 0x20 };
                tile_data_size.write_to(char_buff, o)?;
                0x18u32.write_to(char_buff, o)?;
                if self.is_8_bit {
                    for tile in c {
                        char_buff.write(tile)?;
                    }
                } else {
                    for tile in c {
                        for i in 0..0x20 {
                            let eightbit = (tile[i * 2 + 1] << 4) + tile[i * 2];
                            eightbit.write_to(char_buff, o)?;
                        }
                    }
                }
            }
            NCGRTiles::Lineal(c) => {
                1u32.write_to(char_buff, o)?;
                tile_data_size = c.len() as u32;
                tile_data_size.write_to(char_buff, o)?;
                0x18u32.write_to(char_buff, o)?;
                char_buff.write(c)?;
            }
        }

        // CPOS section
        let ref mut cpos_buff: Vec<u8> = vec![];
        0u32.write_to(cpos_buff, o)?;
        if self.is_8_bit { 0x40u16 } else { 0x20 }.write_to(cpos_buff, o)?;
        tile_data_size.write_to(cpos_buff, o)?;

        let mut out = NDSFile {
            byteorder: o,
            magic: "RGCN".to_string(),
            fname,
            sections: vec![Section {
                magic: "RAHC".to_string(),
                contents: char_buff.clone(),
            }],
        };

        if self.has_cpos {
            out.sections.push(Section {
                magic: "SOPC".to_string(),
                contents: cpos_buff.clone(),
            })
        }

        Ok(out)
    }
}

impl NCGRTiles {
    /// Parses NCGR tile data into an NCGRTiles
    pub fn from_tile_data(
        data: &mut &[u8],
        num_tiles: usize,
        is_lineal: bool,
        is_8_bit: bool,
    ) -> Self {
        if is_lineal {
            Self::Lineal(data.to_vec())
        } else {
            let mut tilesvec = vec![];
            if is_8_bit {
                for _ in 0..num_tiles {
                    let mut tile = vec![];
                    for _ in 0..0x40 {
                        tile.push(u8::read_from(data, ByteOrder::LittleEndian).unwrap());
                    }
                    tilesvec.push(tile);
                }
            } else {
                for _ in 0..num_tiles {
                    let mut tile = vec![];
                    for _ in 0..0x20 {
                        let eightbit = u8::read_from(data, ByteOrder::LittleEndian).unwrap();
                        tile.push(eightbit & 0xF);
                        tile.push(eightbit >> 4);
                    }
                    tilesvec.push(tile);
                }
            }
            Self::Horizontal(tilesvec)
        }
    }

    /// Converts the NCGRTiles into a [Vec<Tile>] to be referred by NSCR
    pub fn to_tiles(&self, is_8_bit: bool) -> Vec<Tile> {
        match self {
            NCGRTiles::Horizontal(c) => c.to_vec(),
            NCGRTiles::Lineal(_) => {
                let imgdata = self.render(is_8_bit, None, 32);
                let height = imgdata.len() / 256 / 8;
                let mut tiles = vec![];
                for i in 0..height {
                    for j in 0..32 {
                        let mut tile = vec![];
                        for k in 0..8 {
                            tile.extend(
                                &imgdata[(i * 8 + k) * 256 + j * 8..(i * 8 + k) * 256 + j * 8 + 7],
                            );
                        }
                        tiles.push(tile);
                    }
                }
                tiles
            }
        }
    }

    /// Converts the NCGRTiles into image data to be displayed
    pub fn render(
        &self,
        is_8_bit: bool,
        range: Option<Range<usize>>,
        render_width: usize,
    ) -> Vec<u8> {
        match self {
            Self::Horizontal(c) => Self::render_tiles(c, range, render_width),
            Self::Lineal(c) => {
                let tile_size = if is_8_bit { 0x40 } else { 0x20 };
                let tile_data = match range {
                    Some(d) => &c[d.start * tile_size..d.end * tile_size],
                    None => &c,
                };
                if is_8_bit {
                    tile_data.to_vec()
                } else {
                    let mut out = vec![];
                    for byte in tile_data {
                        out.push(byte & 0xF);
                        out.push(byte >> 4);
                    }
                    out
                }
            }
        }
    }

    /// Converts a vector of [Tile] into indexed image data to be displayed
    pub fn render_tiles(
        tiles: &Vec<Tile>,
        range: Option<Range<usize>>,
        render_width: usize,
    ) -> Vec<u8> {
        let tiles = match range {
            Some(d) => &tiles[d],
            None => &tiles,
        };
        let mut imgdata: Vec<u8> = vec![];

        let mut current_scanlines = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        for i in 0..tiles.len() {
            let tile = &tiles[i];
            for j in 0..8 {
                let row = &tile[j * 8..(j + 1) * 8];
                current_scanlines[j].extend(row);
            }

            if i % render_width == render_width - 1 {
                for scanline in &mut current_scanlines {
                    imgdata.extend(scanline.to_vec());
                    *scanline = vec![];
                }
            }
        }
        if tiles.len() % render_width != 0 {
            for _ in 0..render_width - (tiles.len() % render_width) {
                for scanline in &mut current_scanlines {
                    for _ in 0..8 {
                        scanline.push(0)
                    }
                }
            }
            for scanline in &mut current_scanlines {
                imgdata.extend(scanline.to_vec());
                *scanline = vec![];
            }
        }
        imgdata
    }

    /// Obtain number of tiles
    pub fn len(&self, is_8_bit: bool) -> usize {
        match self {
            Self::Horizontal(c) => c.len(),
            Self::Lineal(c) => c.len() / if is_8_bit { 0x40 } else { 0x20 },
        }
    }
}
