use crate::{
    error::{Error, Result},
    ndsfile::{NDSFile, Section},
};

#[derive(Debug, Clone)]
pub struct NCER {
    pub sprites: Vec<Vec<NCERCell>>,
    pub bounds: u32,
}

#[derive(Debug, Clone)]
pub enum NCERCell {
    Packed([u8; 3]),
    RotScaleEnabled(RotScaleCell),
    RotScaleDisabled(CellInternals),
}

// most likely not used in NCER due to the rotation/scaling attribute missing
#[derive(Debug, Clone)]
pub struct RotScaleCell {
    _do_not_construct: (),
}

#[derive(Debug, Clone)]
pub struct CellInternals {
    pub x_coord: u8,
    pub y_coord: u8,
    pub disable: bool,
    pub mode: u8,           // 2-bit
    pub mosaic: bool,
    pub is_8_bit: bool,
    pub shape: CellShape,   // 2-bit + 2-bit
    pub unused_attr1: u8,   // 3-bit
    pub flip_x: bool,
    pub flip_y: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellShape {
    Cell8x8,
    Cell16x8,
    Cell8x16,
    Cell16x16,
    Cell32x8,
    Cell8x32,
    Cell32x32,
    Cell32x16,
    Cell16x32,
    Cell64x64,
    Cell64x32,
    Cell32x64,
}

impl NCER {
    pub fn from_ndsfile(file: &NDSFile) -> Result<Self> {
        if file.magic != "RECN" {
            Err(Error::WrongFileKind {
                file: file.fname.to_string(),
                ftype: Some("NCER/NDS cell data".to_string().to_string()),
                expected: "RECN".to_string(),
                got: file.magic.to_string(),
            })?
        }

        //let mut sprites = vec![];
        let mut bounds = 0;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.as_ref() {
                "KBEC" => {
                    todo!("CEBK section")
                }
                "LBAL" => {todo!("LABL section")}
                "TXEU" => {todo!("UEXT section")}
                c => Err(Error::UnknownSection {
                    file: file.fname.clone(),
                    s_name: c.to_string(),
                })?,
            }
        }

        todo!();

    }
}

impl CellShape {
    pub fn new(shape: u8, size: u8) -> Result<Self> {
        use CellShape::*;
        Ok(match (shape, size) {
            (0, 0) => Cell8x8,
            (1, 0) => Cell16x8,
            (2, 0) => Cell8x16,
            (0, 1) => Cell16x16,
            (1, 1) => Cell32x8,
            (2, 1) => Cell8x32,
            (0, 2) => Cell32x32,
            (1, 2) => Cell32x16,
            (2, 2) => Cell16x32,
            (0, 3) => Cell32x32,
            (1, 3) => Cell32x16,
            (2, 3) => Cell16x32,
            _ => Err(Error::Generic(format!(
                "Invalid values for CellFullShape: shape={}, size={}",
                shape, size
            )))?,
        })
    }
}