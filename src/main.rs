use bytestream::ByteOrder;
use std::{fs::File, path::PathBuf};

fn main() -> nuclear::error::Result<()> {
    let proj = nuclear::proj::NuclearProject::new(
        "the super cool project",
        "patataofcourse",
        PathBuf::from("test_files/rockers"),
    )?;

    let mut f = File::open("test_files/rocker_bg.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCLR", &mut f)?;
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(&nds)?;

    proj.add_nclr("rocker_bg", &clr)?;

    Ok(())
}

/*
fn main() -> nuclear::error::Result<()> {
    // Open NCLR file
    let mut f = File::open("test_files/rocker_bg.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCLR", &mut f)?;

    // Export NCLR to palette set
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(&nds)?;
    clr.to_dir("test_files/rocker_bg_pal".into())?;

    // Re-export NCLR file
    let nds = clr.to_ndsfile("rocker_bg.NCLR".to_string(), ByteOrder::LittleEndian)?;
    let mut f_w = File::create("test_files/rocker_bg.out.NCLR")?;
    nds.to_file(&mut f_w)?;

    // Open NCGR/NCBR file
    let mut f = File::open("test_files/rocker_bg.NCGR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCGR", &mut f)?;

    // Export NCGR to tilesheet
    let ref mut f_w = File::create("test_files/rocker_bg.tiles.png")?;
    let cgr = nuclear::img::ncgr::NCGR::from_ndsfile(&nds)?;
    nuclear::img::renderer::Renderer.export_tilesheet(f_w, &clr.palettes[&0], &cgr, 32, false)?;

    // Open NSCR file
    let mut f = File::open("test_files/rocker_bg.NSCR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NSCR", &mut f)?;

    // Export NSCR to image
    let ref mut f_w = File::create("test_files/rocker_bg.png")?;
    let scr = nuclear::img::nscr::NSCR::from_ndsfile(&nds)?;
    nuclear::img::renderer::Renderer.export_tilemap(f_w, &clr, &cgr, &scr)?;

    Ok(())
}
*/
