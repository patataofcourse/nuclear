/*
To eventually be added to Renderer struct
    - Load / unload NCLR
    - Load / unload NCGR
    - Render samples for both of these formats
    - Load / unload NCER, + render its frames
    - Render NANR (from NCLR + NCGR + NCER) and NSCR (from NCLR + NCGR)
*/

pub struct Renderer;

impl Renderer {
    pub fn export_palettes(&self, pal: super::NCLR) {} // stub for documentation
}
