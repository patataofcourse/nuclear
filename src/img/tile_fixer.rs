// tile_fixer.rs
//   Rust adaptation of ShaffySwitcher's R-IQ Tile Fixer program, with multipalette support

use std::{collections::BTreeMap, io::Read};

use crate::{
    error::{Error, Result},
    format::{ncgr::NCGRTiles, nscr::TileRef, ColorBGR555, Tile, NCGR, NCLR, NSCR},
};

use super::png_util::ImgHelper;

#[derive(Eq, Clone, Debug)]
pub struct FixerTile {
    pub pixels: [[ColorBGR555; 8]; 8],
    pub pal: u16,
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
        Ok(Self { pixels, pal: 0 })
    }

    pub fn flip(&self, x: bool, y: bool) -> Self {
        let mut pixels = [[Default::default(); 8]; 8];
        for (j, row) in pixels.iter_mut().enumerate() {
            for (i, pixel) in row.iter_mut().enumerate() {
                *pixel = self.pixels[if y { 7 - j } else { j }][if x { 7 - i } else { i }]
            }
        }
        Self { pixels, pal: 0 }
    }
}

pub fn image_to_tiles<R: Read>(
    img: &mut R,
) -> Result<(Vec<FixerTile>, Vec<TileRef>, usize, usize)> {
    let img = ImgHelper::new(png::Decoder::new(img).read_info()?)?;

    if img.width % 8 != 0 || img.height % 8 != 0 {
        Err(Error::Generic(
            "Image size is not a multiple of 8x8!".to_string(),
        ))?
    }

    let mut tiles: Vec<FixerTile> = vec![];
    let mut tile_refs = vec![];

    // Read tiles left to right, top to bottom
    for j in 0..img.height / 8 {
        for i in 0..img.width / 8 {
            let tile = FixerTile::from_image(&img, i * 8, j * 8)?;
            let mut tile_ref = None;
            'a: for (c, tile_) in tiles.iter().enumerate() {
                for flip_x in [false, true] {
                    for flip_y in [false, true] {
                        if tile == tile_.flip(flip_x, flip_y) {
                            tile_ref = Some(TileRef {
                                tile: c as u16,
                                flip_x,
                                flip_y,
                                palette: 0,
                            });
                            break 'a;
                        }
                    }
                }
            }
            tile_refs.push(tile_ref.unwrap_or_else(|| {
                tiles.push(tile);
                TileRef {
                    tile: tiles.len() as u16 - 1,
                    flip_x: false,
                    flip_y: false,
                    palette: 0,
                }
            }));
        }
    }

    Ok((tiles, tile_refs, img.width, img.height))
}

impl FixerTile {
    pub fn to_indexed_tiles(
        mut ftiles: Vec<Self>,
        tile_refs: Vec<TileRef>,
        size: [usize; 2],
        is_8_bit: bool,
        lineal_mode: bool,
        has_cpos: bool,
        ncbr_ff: bool,
    ) -> Result<(NCLR, NCGR, NSCR)> {
        let mut palettes: BTreeMap<u16, Vec<ColorBGR555>> = BTreeMap::new();
        let mut tile_refs = tile_refs.to_vec();
        for (i, tile) in ftiles.iter_mut().enumerate() {
            let colors = tile.colors();
            if is_8_bit {
                match palettes.get_mut(&0) {
                    Some(c) => {
                        let colors = colors
                            .iter()
                            .filter(|col| !c.contains(col))
                            .collect::<Vec<_>>();
                        if colors.len() + c.len() >= 256 {
                            Err(Error::Generic("Image has too many colors!".to_string()))?
                        }
                        c.extend(colors);
                    }
                    None => {
                        palettes.insert(0, colors);
                    }
                }
            } else {
                let mut num = None;
                for j in 0..16 {
                    match palettes.get_mut(&j) {
                        None => {
                            palettes.insert(j, colors);
                            num = Some(j);
                            break;
                        }
                        Some(c) => {
                            if c.len() > 16 {
                                unreachable!("what")
                            }
                            if c.len() == 16 {
                                continue;
                            }
                            let colors = colors
                                .iter()
                                .filter(|col| !c.contains(col))
                                .collect::<Vec<_>>();
                            if colors.len() + c.len() < 16 {
                                c.extend(colors);
                                num = Some(j);
                                break;
                            }
                        }
                    }
                }
                let Some(num) = num else {Err(Error::Generic("Image has too many colors!".to_string()))?};

                for tile in &mut tile_refs {
                    if tile.tile == i as u16 {
                        tile.palette = num as u8;
                    }
                }
                tile.pal = num;
            }
        }

        let mut color_amt = 0u32;
        for pal in &palettes {
            if pal.1.len() > color_amt as usize {
                color_amt = pal.1.len() as u32;
            }
        }
        let clr = NCLR {
            palettes,
            is_8_bit,
            color_amt,
        };

        let mut tiles = vec![];
        for tile in ftiles {
            tiles.push(tile.apply_palette(clr.palettes.get(&tile.pal).unwrap()));
        }

        let cgr = NCGR {
            tiles: NCGRTiles::from_tiles(tiles, is_8_bit, lineal_mode),
            is_8_bit,
            has_cpos,
            ncbr_ff,
        };

        let scr = NSCR {
            tiles: tile_refs,
            width: size[0] as u16,
            height: size[1] as u16,
        };

        Ok((clr, cgr, scr))
    }

    pub fn colors(&self) -> Vec<ColorBGR555> {
        let mut colors = vec![];
        for row in self.pixels {
            for pixel in row {
                if !colors.contains(&pixel) {
                    colors.push(pixel);
                }
            }
        }
        colors
    }

    pub fn apply_palette(&self, pal: &[ColorBGR555]) -> Tile {
        let mut tile = vec![];
        for row in self.pixels {
            for pixel in row {
                tile.push(
                    pal.iter()
                        .position(|c| c == &pixel)
                        .expect("This should never happen!") as u8,
                )
            }
        }
        tile
    }
}
