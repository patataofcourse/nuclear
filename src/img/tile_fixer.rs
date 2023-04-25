// tile_fixer.rs
//   Rust adaptation of ShaffySwitcher's R-IQ Tile Fixer program, with multipalette support

use std::io::Read;

use crate::{
    error::Result,
    img::{png_util::ImgHelper, ColorBGR555},
};
use png::Reader;

pub struct FixerTile {
    pub pixels: [[ColorBGR555; 8]; 8],
    pub flip_x: bool,
    pub flip_y: bool,
}

impl FixerTile {
    pub fn from_image<R: Read>(img: Reader<R>, x: usize, y: usize) -> Result<Self> {
        let img_data = ImgHelper::new(img)?;

        let mut pixels = [[Default::default(); 8]; 8];
        for (i, column) in pixels.iter_mut().enumerate() {
            for (j, pixel) in column.iter_mut().enumerate() {
                *pixel = ColorBGR555::from_rgb8(img_data.get_pixel(x + i, y + j));
            }
        }
        Ok(Self {
            pixels,
            flip_x: false,
            flip_y: false,
        })
    }

    pub fn flip(&self, x: bool, y: bool) -> Self {
        let flip_x = x ^ self.flip_x;
        let flip_y = y ^ self.flip_y;
        let mut pixels = [[Default::default(); 8]; 8];
        for (i, column) in pixels.iter_mut().enumerate() {
            for (j, pixel) in column.iter_mut().enumerate() {
                *pixel = self.pixels[if flip_x { 7 - i } else { i }][if flip_y { 7 - j } else { j }]
            }
        }
        Self {
            pixels,
            flip_x: x,
            flip_y: y,
        }
    }
}
