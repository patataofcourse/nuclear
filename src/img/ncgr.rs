use crate::{
    error::{Error, Result},
    ndsfile::NDSFile,
};

#[derive(Debug, Clone)]
pub struct NCGR {
    pub is_8_bit: bool,
    pub unk: u32,
    pub tiles: Vec<Vec<u8>>,
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
        unimplemented!();
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
