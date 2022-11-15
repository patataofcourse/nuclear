use crate::{
    error::{Error, Result},
    img::ColorBGR555,
    ndsfile::{NDSFile, Section},
};

use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    ops::Deref,
};

#[derive(Debug, Clone)]
/// NCLR (Nintendo CoLor Resource) palette format
pub struct NCLR {
    /// The palettes themselves, in BGR555 format
    pub palettes: BTreeMap<u16, Vec<ColorBGR555>>,
    /// Indicates whether the file uses 8-bit color (true) or 4-bit color (false)
    pub is_8_bit: bool,
    /// The amount of colors in each palette
    pub color_amt: u32,
}

impl NCLR {
    /// Creates a NCLR struct from the NDSFile given
    pub fn from_ndsfile(file: &NDSFile) -> Result<Self> {
        if file.magic != "RLCN" {
            Err(Error::WrongFileKind {
                file: file.fname.clone(),
                ftype: Some("NCLR/NDS palette".to_string()),
                expected: "RLCN".to_string(),
                got: file.magic.clone(),
            })?
        }

        let mut palettes = None;
        let mut ids = None;
        let mut is_8_bit = false;
        let o = file.byteorder;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.deref() {
                "TTLP" => {
                    let mut palette_vec = vec![];

                    is_8_bit = match u32::read_from(&mut data, o)? {
                        3 => false,
                        4 => true,
                        c => unimplemented!("Unknown color format #{}", c),
                    };
                    u32::read_from(&mut data, o)?; //padding
                    let data_size = u32::read_from(&mut data, o)?;
                    let color_amt = u32::read_from(&mut data, o)?;

                    let mut pos = 0;
                    let mut palette = vec![];
                    while pos < data_size {
                        for _ in 0..color_amt {
                            palette.push(ColorBGR555::read_from(&mut data, o)?);
                            pos += 2;
                        }
                        palette_vec.push(palette);
                        palette = vec![];
                    }

                    if is_8_bit && palette_vec.len() != 1 {
                        Err(Error::MalformedData {
                            file: file.fname.clone(),
                        })?
                    }

                    palettes = Some((palette_vec, color_amt));
                }
                "PMCP" => {
                    ids = Some(vec![]);
                    let pal_count = u16::read_from(&mut data, o)?;

                    // Unknown 6 bytes
                    let mut pcmp_unk = [0u8; 6];
                    data.read(&mut pcmp_unk)?;

                    for _ in 0..pal_count {
                        ids.as_mut().unwrap().push(u16::read_from(&mut data, o)?);
                    }
                }
                c => Err(Error::UnknownSection {
                    file: file.fname.clone(),
                    s_name: c.to_string(),
                })?,
            }
        }
        let mut palette_map = BTreeMap::<u16, Vec<ColorBGR555>>::new();
        let mut color_amt = 0;
        if let Some((pal, amt)) = palettes {
            if let Some(id) = ids {
                if id.len() > pal.len() {
                    Err(Error::MalformedData {
                        file: file.fname.clone(),
                    })?
                }
                for i in 0..id.len() {
                    palette_map.insert(id[i], pal.get(i).unwrap().to_vec());
                }
                color_amt = amt;
            } else {
                Err(Error::MissingRequiredSection {
                    file: file.fname.clone(),
                    s_name: "PCMP".to_string(),
                })?
            }
        } else {
            Err(Error::MissingRequiredSection {
                file: file.fname.clone(),
                s_name: "PLTT".to_string(),
            })?
        }
        Ok(Self {
            is_8_bit,
            palettes: palette_map,
            color_amt,
        })
    }

    /// Exports an NDSFile struct from an NCLR struct
    pub fn to_ndsfile(&self, fname: String, byteorder: ByteOrder) -> Result<NDSFile> {
        let mut pltt_buffer = vec![];
        let mut pcmp_buffer = vec![];

        //PLTT header
        if self.is_8_bit { 4u32 } else { 3u32 }.write_to(&mut pltt_buffer, byteorder)?;
        0u32.write_to(&mut pltt_buffer, byteorder)?;
        (self.palettes.len() as u32 * self.color_amt * 2).write_to(&mut pltt_buffer, byteorder)?;
        self.color_amt.write_to(&mut pltt_buffer, byteorder)?;

        //PCMP header
        (self.palettes.len() as u16).write_to(&mut pcmp_buffer, byteorder)?;
        let pcmp_unk = [0xEFu8, 0xBE, 0x08, 0x00, 0x00, 0x00];
        pcmp_buffer.write(&pcmp_unk)?;

        for (id, palette) in &self.palettes {
            id.write_to(&mut pcmp_buffer, byteorder)?;
            for color in palette {
                color.write_to(&mut pltt_buffer, byteorder)?;
            }
        }
        Ok(NDSFile {
            byteorder,
            magic: "RLCN".to_string(),
            fname,
            sections: vec![
                Section {
                    magic: "TTLP".to_string(),
                    contents: pltt_buffer,
                },
                Section {
                    magic: "PMCP".to_string(),
                    contents: pcmp_buffer,
                },
            ],
        })
    }
}
