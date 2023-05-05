// tile_fixer.rs
//   Rust adaptation of ShaffySwitcher's R-IQ Tile Fixer program, with multipalette support

use std::{collections::BTreeMap, io::Read};

use crate::{
    error::{Error, Result},
    format::{nscr::TileRef, ColorBGR555, Tile, NCGR, NCLR, NSCR},
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

pub fn image_to_tiles<R: Read>(
    img: &mut R,
) -> Result<(Vec<FixerTile>, Vec<TileRef>, usize, usize)> {
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

    Ok((tiles, tile_refs, img.width, img.height))
}

impl FixerTile {
    pub fn to_indexed_tiles(
        ftiles: &[Self],
        tile_refs: &[TileRef],
        size: [usize; 2],
        is_8_bit: bool,
        lineal_mode: bool,
        has_cpos: bool,
        ncbr_ff: bool,
    ) -> Result<(NCLR, NCGR, NSCR)> {
        let mut palettes: BTreeMap<u16, Vec<ColorBGR555>> = BTreeMap::new();
        let mut scr = NSCR {
            width: size[0] as u16,
            height: size[1] as u16,
            tiles: tile_refs.to_vec(),
        };
        for (i, tile) in ftiles.iter().enumerate() {
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
                            println!("{:?}", colors);
                            let colors = colors
                                .iter()
                                .filter(|col| !c.contains(col))
                                .collect::<Vec<_>>();
                            println!("{} {:?}", j, colors);
                            println!("{}", c.len());
                            if colors.len() + c.len() < 16 {
                                c.extend(colors);
                                num = Some(j);
                                break;
                            }
                        }
                    }
                }
                let Some(num) = num else {Err(Error::Generic("Image has too many colors!".to_string()))?};
                for tile in &mut scr.tiles {
                    if tile.tile == i as u16 {
                        tile.palette = num as u8;
                    }
                }
            }
        }

        let mut color_amt = 0u32;
        for pal in &palettes {
            if pal.1.len() > color_amt as usize {
                color_amt = pal.1.len() as u32;
            }
        }

        //TODO: remove this, it's just for testing!!!
        let mut f = std::fs::File::create("test_files/test.nclr").unwrap();
        use crate::ndsfile::NDSFileType;
        NCLR {
            palettes,
            is_8_bit,
            color_amt,
        }
        .to_file(
            &mut f,
            "test.nclr".into(),
            bytestream::ByteOrder::LittleEndian,
        )
        .unwrap();
        todo!("Convert FixerTiles to tiles with palettes");
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

    pub fn apply_palette(&self, pal: &[ColorBGR555], is_8_bit: bool) -> Tile {
        let mut tile = vec![];
        for row in self.pixels {
            if is_8_bit {
                for pixel in row {
                    tile.push(
                        pal.iter()
                            .position(|c| c == &pixel)
                            .expect("This should never happen!") as u8,
                    )
                }
            } else {
                let mut cur_byte = 0u8;
                for (i, pixel) in row.iter().enumerate() {
                    let color = pal
                        .iter()
                        .position(|c| c == pixel)
                        .expect("This should never happen!") as u8;
                    if i % 2 == 0 {
                        cur_byte = color;
                    } else {
                        tile.push(cur_byte & (color << 4))
                    }
                }
            }
        }
        tile
    }
}
