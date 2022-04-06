use super::ColorBGR555;
use crate::{
    error::Result,
    ndsfile::{NDSFile, Section},
};

use std::collections::HashMap;

#[derive(Debug)]
/// NCLR (Nintendo CoLoR) palette format
pub struct NCLR {
    pub palettes: HashMap<u16, Vec<ColorBGR555>>,
    pub pcmp_unk: [u8; 6],
}

impl NCLR {
    //TODO
    pub fn from_ndsfile(file: NDSFile) -> Result<Self> {
        unimplemented!();
    }
}
