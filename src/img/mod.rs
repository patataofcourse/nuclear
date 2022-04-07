use bytestream::{ByteOrder, StreamReader};
use std::io;

pub mod ncgr;
pub mod nclr;
pub mod renderer;

pub use ncgr::NCGR;
pub use nclr::NCLR;

#[derive(Debug, Clone)]
/// Color format the Nintendo DS uses (BGR555)
pub struct ColorBGR555 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub x: bool,
}

impl ColorBGR555 {
    pub fn to_rgb888(&self) -> [u8; 3] {
        [self.r * 0x8, self.g * 0x8, self.b * 0x8]
    }
}

impl StreamReader for ColorBGR555 {
    fn read_from<R: io::Read>(f: &mut R, o: ByteOrder) -> io::Result<Self> {
        let num = u16::read_from(f, o)?;
        Ok(Self {
            r: (num & 0x1F) as u8,
            g: (num >> 5 & 0x1F) as u8,
            b: (num >> 10 & 0x1F) as u8,
            x: num >> 15 != 0,
        })
    }
}
