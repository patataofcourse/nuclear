pub mod nclr;

#[derive(Debug)]
/// Color format the Nintendo DS uses (BGR555)
pub struct ColorBGR555 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub x: bool,
}
