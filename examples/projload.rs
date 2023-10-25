use nuclear::{error::Result, img::png_util, proj::NuclearProject};
use std::fs::File;

fn main() -> Result<()> {
    let proj = NuclearProject::load_from_file("test_files/rockers")?;

    let nclr = proj.get_nclr("rocker_bg")?.unwrap();
    let ncgr = proj.get_ncgr("rocker_bg")?.unwrap();
    let nscr = proj.get_nscr("rocker_bg")?.unwrap();

    let mut f = File::create("test_files/a.png")?;
    png_util::export_tilemap(&mut f, &nclr, &ncgr, &nscr)?;
    Ok(())
}
