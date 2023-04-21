use crate::error::{Error, Result};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::{
    fmt::{self, Debug, Formatter},
    io::{Read, Seek, SeekFrom, Write},
};

#[derive(Clone)]
pub struct NDSFile {
    pub fname: String,
    pub magic: String,
    pub byteorder: ByteOrder,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub magic: String,
    pub contents: Vec<u8>,
}

impl NDSFile {
    pub fn from_file<F: Read>(fname: &str, f: &mut F) -> Result<Self> {
        let mut magic = [0u8; 4];
        f.read_exact(&mut magic)?;

        let mut bom = [0u8; 2];
        f.read_exact(&mut bom)?;
        let o = match bom {
            [0xFF, 0xFE] => ByteOrder::LittleEndian,
            [0xFE, 0xFF] => ByteOrder::BigEndian,
            _ => Err(Error::InvalidBOM {
                file: fname.to_string(),
            })?,
        };

        u16::read_from(f, o)?; // 0x0001
        u32::read_from(f, o)?; // Full filesize, we can discard it here
        u16::read_from(f, o)?; // Header size, always 0x10

        let section_count = u16::read_from(f, o)?;
        let mut sections = vec![];
        for _ in 0..section_count {
            let mut s_magic = [0u8; 4];
            f.read_exact(&mut s_magic)?;
            let size = u32::read_from(f, o)? - 0x08;
            let mut s_contents = vec![];
            for _ in 0..size {
                s_contents.push(u8::read_from(f, o)?)
            }
            sections.push(Section {
                magic: String::from_utf8(s_magic.into()).unwrap(),
                contents: s_contents,
            });
        }

        Ok(Self {
            fname: fname.to_string(),
            magic: String::from_utf8(magic.into()).unwrap(),
            sections,
            byteorder: o,
        })
    }

    pub fn to_file<F: Write + Seek>(&self, f: &mut F) -> Result<()> {
        f.write_all(self.magic.as_bytes())?;
        f.write_all(match self.byteorder {
            ByteOrder::BigEndian => &[0xFE, 0xFF],
            ByteOrder::LittleEndian => &[0xFF, 0xFE],
        })?;

        1u16.write_to(f, self.byteorder)?;
        0u32.write_to(f, self.byteorder)?; // This will be written later with the entire filesize
        0x10u16.write_to(f, self.byteorder)?;
        (self.sections.len() as u16).write_to(f, self.byteorder)?; // Section count

        for section in &self.sections {
            f.write_all(section.magic.as_bytes())?;
            (section.contents.len() as u32 + 0x8).write_to(f, self.byteorder)?;
            f.write_all(&section.contents)?;
        }

        let file_size = f.stream_position()? as u32;
        f.seek(SeekFrom::Start(0x8))?;
        file_size.write_to(f, self.byteorder)?;

        Ok(())
    }
}

impl Debug for NDSFile {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("NDSFile")
            .field("magic", &self.magic)
            .field(
                "byteorder",
                match self.byteorder {
                    ByteOrder::BigEndian => &"big",
                    ByteOrder::LittleEndian => &"little",
                },
            )
            .field("sections", &self.sections)
            .finish()
    }
}
