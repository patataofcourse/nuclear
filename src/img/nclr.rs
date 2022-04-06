use super::ColorBGR555;
use crate::{
    error::{Error, Result},
    ndsfile::NDSFile,
};

use bytestream::StreamReader;
use png::{BitDepth, ColorType, Encoder};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Read},
    ops::Deref,
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Clone)]
/// NCLR (Nintendo CoLoR) palette format
pub struct NCLR {
    pub palettes: HashMap<u16, Vec<ColorBGR555>>,
    pub pcmp_unk: [u8; 6],
    pub is_8_bit: bool,
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
        let mut ids: Option<Vec<u16>> = None; //TODO: Remove type param
        let mut pcmp_unk = [0u8; 6];
        let mut is_8_bit = false;
        let o = file.byteorder;

        for section in &file.sections {
            let mut data: &[u8] = &section.contents;
            match section.magic.deref() {
                "TTLP" => {
                    palettes = Some(vec![]);

                    is_8_bit = u32::read_from(&mut data, o)? == 4;
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
                        palettes.as_mut().unwrap().push(palette);
                        palette = vec![];
                    }
                }
                "PMCP" => {
                    ids = Some(vec![]);
                    let pal_count = u16::read_from(&mut data, o)?;
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
        let mut palette_map = HashMap::<u16, Vec<ColorBGR555>>::new();
        if let Some(pal) = palettes {
            if let Some(id) = ids {
                if id.len() > pal.len() {
                    Err(Error::MalformedData {
                        file: file.fname.clone(),
                    })?
                }
                for i in 0..id.len() {
                    palette_map.insert(id[i], pal.get(i).unwrap().to_vec());
                }
            } else {
                Err(Error::MissingRequiredSection {
                    file: file.fname.clone(),
                    s_name: "TTLP".to_string(),
                })?
            }
        } else {
            Err(Error::MissingRequiredSection {
                file: file.fname.clone(),
                s_name: "TTLP".to_string(),
            })?
        }
        Ok(Self {
            is_8_bit,
            pcmp_unk,
            palettes: palette_map,
        })
    }

    ///Exports a folder with all the palettes in it, in PNG format
    pub fn to_dir(&self, dir: PathBuf) -> Result<()> {
        fs::create_dir_all(&dir)?;
        let height = if self.is_8_bit { 16 } else { 1 };
        let depth = if self.is_8_bit {
            BitDepth::Eight
        } else {
            BitDepth::Four
        };
        for (id, palette) in &self.palettes {
            let mut fpath = dir.clone();
            fpath.push(PathBuf::from_str(&format!("{}.png", id))?);
            let f = File::create(fpath)?;

            let ref mut w = BufWriter::new(f);
            let mut encoder = Encoder::new(w, 16, height);
            encoder.set_color(ColorType::Indexed);
            encoder.set_depth(depth);
            let mut pal = vec![];
            for color in palette {
                pal.extend(color.to_rgb888());
            }
            encoder.set_palette(pal);
            let mut writer = encoder.write_header()?;
            let data_8bit = (0..=0xFFu8).collect::<Vec<u8>>();
            let data_4bit = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
            writer.write_image_data(if self.is_8_bit {
                &data_8bit
            } else {
                &data_4bit
            })?;
            writer.finish()?;
        }
        Ok(())
    }
}
