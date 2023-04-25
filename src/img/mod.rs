use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::io;

pub mod ncgr;
pub mod nclr;
pub mod nscr;

//pub mod nanr;
//pub mod ncer;

/// Helpers for the png format
pub mod png_util;

/// Rust adaptation of ShaffySwitcher's R-IQ Tile Fixer program,
/// used to convert from image to NDS/GBA tileset and tilemap data
pub mod tile_fixer;

pub use ncgr::{Tile, NCGR};
pub use nclr::NCLR;
pub use nscr::NSCR;

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
/// Color format the Nintendo DS uses (BGR555)
pub struct ColorBGR555 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorBGR555 {
    /// Converts the color to the RGB8 (24-bit) format
    pub fn to_rgb8(&self) -> [u8; 3] {
        [self.r * 0x8, self.g * 0x8, self.b * 0x8]
    }

    /// Converts the color to the RGBA8 (32-bit) format
    pub fn to_rgba8(&self) -> [u8; 4] {
        [self.r * 0x8, self.g * 0x8, self.b * 0x8, 255]
    }

    /// Gets a BGR555 color from the RGB8 (24-bit) format
    pub fn from_rgb8(color: [u8; 3]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}

impl StreamReader for ColorBGR555 {
    fn read_from<R: io::Read>(f: &mut R, o: ByteOrder) -> io::Result<Self> {
        let num = u16::read_from(f, o)?;
        Ok(Self {
            r: (num & 0x1F) as u8,
            g: (num >> 5 & 0x1F) as u8,
            b: (num >> 10 & 0x1F) as u8,
        })
    }
}

impl StreamWriter for ColorBGR555 {
    #[rustfmt::skip]
    fn write_to<W: io::Write>(&self, f: &mut W, o: ByteOrder) -> io::Result<()> {
        let num = (self.r as u16 & 0x1F) +
            ((self.g as u16 & 0x1F) << 5) +
            ((self.b as u16 & 0x1F) << 10);
        num.write_to(f, o)
    }
}
