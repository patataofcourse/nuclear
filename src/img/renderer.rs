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
use std::{io::Write, path::PathBuf};

pub struct Renderer;

impl Renderer {
    pub fn export_palettes(&self, pal: NCLR, dir: PathBuf) -> Result<()> {
        unimplemented!();
    }

    pub fn tiles_to_image_data(
        &self,
        tiles: &Vec<Tile>,
        is_8_bit: bool,
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
                if is_8_bit {
                    let row = &tile[j * 8..(j + 1) * 8];
                    current_scanlines[j].write(row)?;
                } else {
                    for k in 0..4 {
                        let byte = tile[4 * j + k];
                        current_scanlines[j].push((byte & 0xF0) >> 4);
                        current_scanlines[j].push(byte & 0x0F);
                    }
                }
            }

            if i % width == width - 1 {
                for scanline in &current_scanlines {
                    imgdata.write(&scanline)?;
                }
            }
        }
        Ok(imgdata)
    }

    pub fn export_tilesheet<W: Write>(
        &self,
        f: &mut W,
        pal: Vec<ColorBGR555>,
        tiles: NCGR,
    ) -> Result<()> {
        unimplemented!()
    }
}
