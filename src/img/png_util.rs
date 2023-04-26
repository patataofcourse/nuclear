// when i originally made this i had a completely different idea of how nuclear would go,
// so right now it's just here for the examples

// that said, export_image *is* useful

use crate::{
    error::{Error, Result},
    img::{ColorBGR555, NCGR, NCLR, NSCR},
};
use bytestream::{ByteOrder, StreamReader};
use png::{BitDepth, ColorType, Encoder, Reader};
use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::PathBuf,
    str::FromStr,
};

//TODO: make this take a Vec<&mut impl W> instead of just always using fs
/// Exports a folder with all the palettes in it, in PNG format
pub fn export_palettes(pal: &NCLR, dir: PathBuf) -> Result<()> {
    fs::create_dir_all(&dir)?;
    let height = if pal.is_8_bit { 16 } else { 1 };
    let depth = if pal.is_8_bit {
        BitDepth::Eight
    } else {
        BitDepth::Four
    };
    for (id, palette) in &pal.palettes {
        let mut fpath = dir.clone();
        fpath.push(PathBuf::from_str(&format!("{}.png", id))?);
        let f = File::create(fpath)?;

        let w = &mut BufWriter::new(f);
        let mut encoder = Encoder::new(w, 16, height);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(depth);
        let mut p = vec![];
        for color in palette {
            p.extend(color.to_rgb8());
        }
        encoder.set_palette(p);
        let mut writer = encoder.write_header()?;
        let data_8bit = (0..=0xFFu8).collect::<Vec<u8>>();
        let data_4bit = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        writer.write_image_data(if pal.is_8_bit { &data_8bit } else { &data_4bit })?;
        writer.finish()?;
    }
    Ok(())
}

pub fn export_tilesheet<W: Write>(
    f: &mut W,
    pal: &Vec<ColorBGR555>,
    tiles: &NCGR,
    width: usize,
    transparency: bool,
) -> Result<()> {
    let img_data = tiles.tiles.render(tiles.is_8_bit, None, width);
    let height = (img_data.len() / 0x8 / width) as u32;

    let w = &mut BufWriter::new(f);
    let mut encoder = Encoder::new(w, width as u32 * 8, height);

    if transparency {
        let mut trns = vec![0];
        trns.extend(vec![255; pal.len() - 1]);
        encoder.set_trns(trns);
    }

    encoder.set_color(ColorType::Indexed);
    encoder.set_depth(BitDepth::Eight);

    let mut palette = vec![];
    for color in pal {
        palette.extend(color.to_rgb8());
    }
    encoder.set_palette(palette);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&img_data)?;
    writer.finish()?;
    Ok(())
}

pub fn export_tilemap<W: Write>(f: &mut W, pal: &NCLR, tiles: &NCGR, map: &NSCR) -> Result<()> {
    let w = &mut BufWriter::new(f);
    let mut encoder = Encoder::new(w, map.width as u32, map.height as u32);

    encoder.set_color(ColorType::Rgb);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&map.render(pal, tiles).unwrap())?;

    Ok(())
}

pub fn export_image<W: Write>(
    f: &mut W,
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
) -> Result<()> {
    let w = &mut BufWriter::new(f);
    let mut encoder = Encoder::new(w, width, height);

    encoder.set_color(color_type);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;

    Ok(())
}

pub struct ImgHelper {
    pub pixels: Vec<[u8; 3]>,
    pub width: usize,
    pub height: usize,
}

impl ImgHelper {
    pub fn new<R: Read>(mut img: Reader<R>) -> Result<Self> {
        let mut image = vec![0u8; img.output_buffer_size()];
        img.next_frame(&mut image)?;
        let info = img.info();
        let mut data: &[u8] = image.as_ref();

        let mut pixels = vec![];
        while !data.is_empty() {
            match (info.color_type, info.bit_depth) {
                (ColorType::Rgb | ColorType::Rgba, BitDepth::Eight) => {
                    pixels.push([
                        u8::read_from(&mut data, ByteOrder::LittleEndian)?,
                        u8::read_from(&mut data, ByteOrder::LittleEndian)?,
                        u8::read_from(&mut data, ByteOrder::LittleEndian)?,
                    ]);
                    if info.color_type == ColorType::Rgba {
                        u8::read_from(&mut data, ByteOrder::LittleEndian)?;
                    }
                }
                _ => Err(Error::Generic(
                    "Unsupported PNG format - must be RGB / RGBA and 8bpp".to_string(),
                ))?,
            }
        }

        Ok(Self {
            pixels,
            width: info.width as usize,
            height: info.height as usize,
        })
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 3] {
        self.pixels[x + y * self.width]
    }
}
