use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::{
    error::{Error, Result},
    ndsfile::{NDSFile, NDSFileType},
};
use bytestream::StreamReader;

#[derive(Debug, Clone)]
pub struct NCER {
    pub cells: Vec<NCERCell>,
    pub mapping_mode: u32,
    pub bounds: u32,
    pub labels: Vec<String>,
    pub uext: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct NCERCell {
    pub objects: Vec<OAMObject>,
    pub bounding_box: Option<CellBounds>,
    pub attrs: u16,
}

#[derive(Debug, Clone)]
pub struct CellBounds {
    pub min_x: u16,
    pub max_x: u16,
    pub min_y: u16,
    pub max_y: u16,
}

#[derive(Debug, Clone)]
pub enum OAMObject {
    RotScaleDisabled(ObjectInternals),
    RotScaleEnabled(RotScaleObject),
}

// TODO: is this even possible with NCER??? delete if not
#[derive(Debug, Clone)]
pub struct RotScaleObject {
    _do_not_construct: (),
}

#[derive(Debug, Clone)]
pub struct ObjectInternals {
    pub tile: u16,   // 10-bit
    pub palette: u8, // 4-bit, unused in 256-color
    pub is_8_bit: bool,

    pub pos_x: u16, // 9-bit
    pub pos_y: u8,
    pub shape: ObjectShape, // 2-bit + 2-bit
    pub flip_x: bool,
    pub flip_y: bool,

    pub priority: u8, // 2-bit
    pub disable: bool,
    pub mode: u8, // 2-bit
    pub mosaic: bool,

    pub unused_attr1: u8, // 3-bit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectShape {
    Object8x8,
    Object16x8,
    Object8x16,
    Object16x16,
    Object32x8,
    Object8x32,
    Object32x32,
    Object32x16,
    Object16x32,
    Object64x64,
    Object64x32,
    Object32x64,
}

impl NDSFileType for NCER {
    fn from_ndsfile(file: &NDSFile) -> Result<Self> {
        if file.magic != "RECN" {
            Err(Error::WrongFileKind {
                file: file.fname.to_string(),
                ftype: Some("NCER/NDS cell data".to_string().to_string()),
                expected: "RECN".to_string(),
                got: file.magic.to_string(),
            })?
        }

        //let mut sprites = vec![];
        let o = file.byteorder;
        let mut labels = vec![];
        let mut uext = vec![];
        let mut cells = vec![];

        for section in &file.sections {
            let mut data = Cursor::new(section.contents.as_slice());
            match section.magic.as_ref() {
                "KBEC" => {
                    let num_cells = u16::read_from(&mut data, o)?;
                    let has_bounding_box = u16::read_from(&mut data, o)? & 1 == 1;
                    let cell_data_offset = u32::read_from(&mut data, o)?;
                    let mapping_mode = u32::read_from(&mut data, o)?;
                    let dma_data_offset = u32::read_from(&mut data, o)?;
                    u32::read_from(&mut data, o)?; // unused
                    let ucat_data_offset = u32::read_from(&mut data, o)?;

                    let oam_data_offset = cell_data_offset
                        + (num_cells as u32 * (if has_bounding_box { 16 } else { 8 }));

                    data.seek(SeekFrom::Start(cell_data_offset as u64))?;

                    for _ in 0..num_cells {
                        let num_oams = u16::read_from(&mut data, o)?;
                        let cell_attrs = u16::read_from(&mut data, o)?;
                        let objects_offset = u32::read_from(&mut data, o)?;

                        let bounding_box = if has_bounding_box {
                            Some(CellBounds {
                                max_x: u16::read_from(&mut data, o)?,
                                max_y: u16::read_from(&mut data, o)?,
                                min_x: u16::read_from(&mut data, o)?,
                                min_y: u16::read_from(&mut data, o)?,
                            })
                        } else {
                            None
                        };

                        let cur_pos = data.stream_position()?;

                        data.seek(SeekFrom::Start((oam_data_offset + objects_offset) as u64))?;

                        let mut objects = vec![];

                        for _j in 0..num_oams {
                            let oam = [
                                u16::read_from(&mut data, o)?,
                                u16::read_from(&mut data, o)?,
                                u16::read_from(&mut data, o)?,
                            ];

                            let object = ObjectInternals::from_attributes(&oam)?;
                            objects.push(OAMObject::RotScaleDisabled(object));
                        }

                        data.seek(SeekFrom::Start(cur_pos))?;

                        cells.push(NCERCell {
                            objects,
                            bounding_box,
                            attrs: cell_attrs,
                        });
                    }

                    todo!("CEBK section: dma, ucat")
                }
                "LBAL" => {
                    let mut offsets = vec![];
                    let mut ptr = u32::read_from(&mut data, o)?;

                    while ptr <= 0xFFFF {
                        offsets.push(ptr);
                        ptr = u32::read_from(&mut data, o)?;
                    }

                    data.seek(SeekFrom::Current(-4))?;

                    let pos = data.stream_position()?;

                    for offset in offsets {
                        data.seek(SeekFrom::Start(pos + offset as u64))?;

                        let mut string = String::new();

                        loop {
                            let c = u8::read_from(&mut data, o)?;
                            if c == 0 {
                                break;
                            }

                            string.push(char::from_u32(c.into()).unwrap());
                        }

                        labels.push(string);
                    }

                    println!("{:?}", labels);
                }
                "TXEU" => {
                    data.read_to_end(&mut uext)?;
                }
                c => Err(Error::UnknownSection {
                    file: file.fname.clone(),
                    s_name: c.to_string(),
                })?,
            }
        }

        todo!();
    }

    fn to_ndsfile(&self, fname: String, order: bytestream::ByteOrder) -> Result<NDSFile> {
        todo!()
    }
}

impl ObjectShape {
    pub fn new(shape: u8, size: u8) -> Result<Self> {
        use ObjectShape::*;
        Ok(match (shape, size) {
            (0, 0) => Object8x8,
            (1, 0) => Object16x8,
            (2, 0) => Object8x16,
            (0, 1) => Object16x16,
            (1, 1) => Object32x8,
            (2, 1) => Object8x32,
            (0, 2) => Object32x32,
            (1, 2) => Object32x16,
            (2, 2) => Object16x32,
            (0, 3) => Object32x32,
            (1, 3) => Object32x16,
            (2, 3) => Object16x32,
            _ => Err(Error::Generic(format!(
                "Invalid values for CellFullShape: shape={}, size={}",
                shape, size
            )))?,
        })
    }
}

impl ObjectInternals {
    pub fn from_attributes(attr: &[u16; 3]) -> Result<Self> {
        if attr[0] & 0x100 != 0 {
            return Err(Error::UnimplementedFeature(
                "Rot/Scale OAM attributes".to_string(),
            ));
        }

        // attr 0
        let pos_y = attr[0] as u8;
        let disable = attr[0] & 0x200 != 0;
        let mode = ((attr[0] >> 10) & 3) as u8;
        let mosaic = attr[0] & 0x1000 != 0;
        let is_8_bit = attr[0] & 0x2000 != 0;
        let shape = (attr[0] >> 14) as u8;

        // attr 1
        let pos_x = attr[1] & 0x1FF;
        let unused = ((attr[1] >> 9) & 7) as u8;
        let flip_x = attr[1] & 0x1000 != 0;
        let flip_y = attr[1] & 0x2000 != 0;
        let size = (attr[1] >> 14) as u8;

        // attr 2
        let tile = attr[2] & 0x3FF;
        let priority = ((attr[2] >> 10) & 3) as u8;
        let palette = (attr[2] >> 12) as u8;

        Ok(ObjectInternals {
            tile,
            palette,
            is_8_bit,
            pos_x,
            pos_y,
            shape: ObjectShape::new(shape, size)?,
            flip_x,
            flip_y,
            priority,
            disable,
            mode,
            mosaic,
            unused_attr1: unused,
        })
    }

    pub fn to_attributes(&self) -> [u16; 3] {
        todo!();
    }
}
