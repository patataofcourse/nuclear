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
    img::{ColorBGR555, Tile, NCGR, NCLR},
};
use png::{BitDepth, ColorType, Encoder};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct Renderer;

impl Renderer {
    pub fn export_palettes(&self, pal: NCLR, dir: PathBuf) -> Result<()> {
        unimplemented!();
    }

    pub fn tiles_to_image_data(
        &self,
        tiles: &Vec<Tile>,
        width: usize, // width IN TILES!!!
    ) -> Result<Vec<u8>> {
        let mut imgdata = vec![];

        let mut current_scanlines = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        for i in 0..tiles.len() {
            let tile = &tiles[i];
            for j in 0..8 {
                let row = &tile[j * 8..(j + 1) * 8];
                current_scanlines[j].write(row)?;
            }

            if i % width == width - 1 {
                for scanline in &mut current_scanlines {
                    imgdata.write(&scanline)?;
                    *scanline = vec![];
                }
            }
        }
        if tiles.len() % width != 0 {
            for _ in 0..width - (tiles.len() % width) {
                for scanline in &mut current_scanlines {
                    for _ in 0..8 {
                        scanline.push(0)
                    }
                }
            }
            for scanline in &mut current_scanlines {
                imgdata.write(&scanline)?;
                *scanline = vec![];
            }
        }
        Ok(imgdata)
    }

    pub fn export_tilesheet<W: Write>(
        &self,
        f: &mut W,
        pal: &Vec<ColorBGR555>,
        tiles: &NCGR,
        width: usize,
    ) -> Result<()> {
        let rendered_tiles = tiles.tiles.render_tiles(tiles.is_8_bit, None, 2);
        let height = ((rendered_tiles.len() / width) as u32
            + if rendered_tiles.len() % width != 0 {
                1
            } else {
                0
            })
            * 8;

        let ref mut w = BufWriter::new(f);
        let mut encoder = Encoder::new(w, width as u32 * 8, height);

        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);

        let mut palette = vec![];
        for color in pal {
            palette.extend(color.to_rgb888());
        }
        encoder.set_palette(palette);
        let mut writer = encoder.write_header()?;

        let img_data = self.tiles_to_image_data(&rendered_tiles, width)?;

        writer.write_image_data(&img_data)?;
        writer.finish()?;
        Ok(())
    }
}
