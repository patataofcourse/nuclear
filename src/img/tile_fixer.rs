// tile_fixer.rs
//   Rust adaptation of ShaffySwitcher's R-IQ Tile Fixer program, with multipalette support

use std::io::Read;

use crate::{
    error::{Error, Result},
    format::{nscr::TileRef, ColorBGR555},
};

use super::png_util::ImgHelper;

#[derive(Eq, Clone, Debug)]
pub struct FixerTile {
    pub pixels: [[ColorBGR555; 8]; 8],
    pub flip_x: bool,
    pub flip_y: bool,
}

impl PartialEq for FixerTile {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels
    }
}

impl FixerTile {
    pub fn from_image(img: &ImgHelper, x: usize, y: usize) -> Result<Self> {
        let mut pixels = [[Default::default(); 8]; 8];
        for (j, row) in pixels.iter_mut().enumerate() {
            for (i, pixel) in row.iter_mut().enumerate() {
                *pixel = ColorBGR555::from_rgb8(img.get_pixel(x + i, y + j));
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
        for (j, row) in pixels.iter_mut().enumerate() {
            for (i, pixel) in row.iter_mut().enumerate() {
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

pub fn image_to_tiles<R: Read>(img: &mut R) -> Result<(Vec<FixerTile>, Vec<TileRef>)> {
    let img = ImgHelper::new(png::Decoder::new(img).read_info()?)?;

    if img.width % 8 != 0 || img.height % 8 != 0 {
        Err(Error::Generic(
            "Image size is not a multiple of 8x8!".to_string(),
        ))?
    }

    let mut tiles = vec![];
    let mut tile_refs = vec![];

    // Read tiles left to right, top to bottom
    for j in 0..img.height / 8 {
        for i in 0..img.width / 8 {
            let tile = FixerTile::from_image(&img, i * 8, j * 8)?;
            let mut tile_ref = None;
            for (c, tile_) in tiles.iter().enumerate() {
                //TODO: reduce the size of this thing
                if tile == *tile_ {
                    tile_ref = Some(TileRef {
                        tile: c as u16,
                        flip_x: false,
                        flip_y: false,
                        palette: 0,
                    });
                    break;
                } else if tile == tile_.flip(true, false) {
                    tile_ref = Some(TileRef {
                        tile: c as u16,
                        flip_x: true,
                        flip_y: false,
                        palette: 0,
                    });
                    break;
                } else if tile == tile_.flip(false, true) {
                    tile_ref = Some(TileRef {
                        tile: c as u16,
                        flip_x: false,
                        flip_y: true,
                        palette: 0,
                    });
                    break;
                } else if tile == tile_.flip(true, true) {
                    tile_ref = Some(TileRef {
                        tile: c as u16,
                        flip_x: true,
                        flip_y: true,
                        palette: 0,
                    });
                    break;
                }
            }
            tile_refs.push(tile_ref.unwrap_or_else(|| {
                tiles.push(tile);
                TileRef {
                    tile: tiles.len() as u16,
                    flip_x: false,
                    flip_y: false,
                    palette: 0,
                }
            }));
        }
    }

    Ok((tiles, tile_refs))
}
