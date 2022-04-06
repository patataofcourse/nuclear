use crate::error::Result;
use bytestream::{ByteOrder, StreamReader};
use std::{
    fmt::{self, Debug, Formatter},
    io::Read,
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
        f.read(&mut magic)?;

        let mut bom = [0u8; 2];
        f.read(&mut bom)?;
        let o = match bom {
            [0xFF, 0xFE] => ByteOrder::LittleEndian,
            [0xFE, 0xFF] => ByteOrder::BigEndian,
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
