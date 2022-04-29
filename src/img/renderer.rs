/*
To eventually be added to Renderer struct
    - Load / unload NCLR
    - Load / unload NCGR
    - Render samples for both of these formats
    - Load / unload NCER, + render its frames
    - Render NANR (from NCLR + NCGR + NCER) and NSCR (from NCLR + NCGR)
*/

use crate::{
    error::Result,
    img::{ColorBGR555, NCGR, NCLR},
};
use png::{BitDepth, ColorType, Encoder};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct Renderer;

impl Renderer {
    pub fn export_palettes(&self, pal: NCLR, dir: PathBuf) -> Result<()> {
        todo!();
    }

    pub fn export_tilesheet<W: Write>(
        &self,
        f: &mut W,
        pal: &Vec<ColorBGR555>,
        tiles: &NCGR,
        width: usize,
        transparency: bool,
    ) -> Result<()> {
        let img_data = tiles.tiles.render(tiles.is_8_bit, None, width);
        let height = (img_data.len() / 0x8 / width) as u32;

        let ref mut w = BufWriter::new(f);
        let mut encoder = Encoder::new(w, width as u32 * 8, height);

        if transparency {
            let mut trns = vec![0];
            for _ in 0..pal.len() - 1 {
                trns.push(255);
            }
            encoder.set_trns(trns);
        }

        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);

        let mut palette = vec![];
        for color in pal {
            palette.extend(color.to_rgb888());
        }
        encoder.set_palette(palette);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&img_data)?;
        writer.finish()?;
        Ok(())
    }
}
