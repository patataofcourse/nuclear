use bytestream::{ByteOrder, StreamReader};
use std::{
    fmt::{self, Debug, Formatter},
    io::{Read, Result as IOResult},
};

pub struct NDSFile {
    pub magic: [u8; 4],
    pub byteorder: ByteOrder,
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub enum Section {
    RawSection { magic: [u8; 4], contents: Vec<u8> },
    ParsedSection(Box<dyn SectionType>),
}

pub trait SectionType: Debug {
    fn magic() -> [u8; 4]
    where
        Self: Sized;
    fn to_raw_section(&self) -> Vec<u8>;
    fn from_raw_section(raw: Vec<u8>) -> Self
    where
        Self: Sized;
}

impl NDSFile {
    pub fn from_file<F: Read>(f: &mut F) -> IOResult<Self> {
        let mut magic = [0u8; 4];
        f.read(&mut magic)?;

        let mut bom = [0u8; 2];
        f.read(&mut bom)?;
        let o = match bom {
            [0xFF, 0xFE] => ByteOrder::BigEndian,
            [0xFE, 0xFF] => ByteOrder::LittleEndian,
            _ => panic!("Incorrect Byte Order Mark"),
        };
        drop(bom);

        u16::read_from(f, o)?; // 0x0001
        u32::read_from(f, o)?; // Full filesize, we can discard it here
        u16::read_from(f, o)?; // 0x0010

        let section_count = u16::read_from(f, o)?;
        let mut sections = vec![];
        for _ in 0..section_count {
            let mut s_magic = [0u8; 4];
            f.read(&mut s_magic)?;
            let size = u32::read_from(f, o)?;
            let mut s_contents = vec![];
            for _ in 0..size {
                s_contents.push(u8::read_from(f, o)?)
            }
            sections.push(Section::RawSection {
                magic: s_magic,
                contents: s_contents,
            });
        }

        Ok(Self {
            magic,
            sections,
            byteorder: o,
        })
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
