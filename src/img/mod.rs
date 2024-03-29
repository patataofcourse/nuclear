use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::io;

pub mod ncgr;
pub mod nclr;
pub mod nscr;

//pub mod nanr;
//pub mod ncer;

/// Only kept for the examples, renders different formats to .png
pub mod export;

pub use ncgr::{Tile, NCGR};
pub use nclr::NCLR;
pub use nscr::NSCR;

#[derive(Debug, Clone, Default)]
/// Color format the Nintendo DS uses (BGR555)
pub struct ColorBGR555 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    /// Unused bit, kept just in case
    pub x: bool,
}

impl ColorBGR555 {
    /// Converts the color to the RGB888 (24-bit) format
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

impl StreamWriter for ColorBGR555 {
    #[rustfmt::skip]
    fn write_to<W: io::Write>(&self, f: &mut W, o: ByteOrder) -> io::Result<()> {
        let num = (self.r as u16 & 0x1F) +
            ((self.g as u16 & 0x1F) << 5) +
            ((self.b as u16 & 0x1F) << 10) +
            if self.x { 1 << 15 } else { 0 };
        num.write_to(f, o)
    }
}
