use bytestream::ByteOrder;
use std::{fs::File, path::PathBuf};

fn main() -> nuclear::error::Result<()> {
    let mut proj = nuclear::proj::NuclearProject::new(
        "the super cool project",
        "patataofcourse",
        PathBuf::from("test_files/rockers"),
    )?;

    let mut f = File::open("test_files/rocker_bg.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCLR", &mut f)?;
    let clr = nuclear::img::NCLR::from_ndsfile(&nds)?;

    let mut f = File::open("test_files/rocker_bg.NCGR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCGR", &mut f)?;
    let cgr = nuclear::img::NCGR::from_ndsfile(&nds)?;

    proj.insert_nclr("rocker_bg", &clr)?;
    proj.insert_ncgr("rocker_bg", &cgr)?;

    let mut f = File::open("test_files/rocker.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCLR", &mut f)?;
    let clr = nuclear::img::NCLR::from_ndsfile(&nds)?;

    let mut f = File::open("test_files/rocker.NCBR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCBR", &mut f)?;
    let cgr = nuclear::img::NCGR::from_ndsfile(&nds)?;

    proj.insert_nclr("rocker", &clr)?;
    proj.insert_ncgr("rocker", &cgr)?;

    let nds = proj
        .get_ncgr("rocker")?
        .unwrap()
        .to_ndsfile("rocker.NCBR".to_string(), ByteOrder::LittleEndian)?;
    let ref mut f_w = File::create("test_files/rocker.proj.NCBR")?;
    nds.to_file(f_w)?;

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

    // Re-export NCGR file
    let nds = cgr.to_ndsfile("rocker_bg.NCGR".to_string(), ByteOrder::LittleEndian)?;
    let mut f_w = File::create("test_files/rocker_bg.out.NCGR")?;
    nds.to_file(&mut f_w)?;

    // Open NSCR file
    let mut f = File::open("test_files/rocker_bg.NSCR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NSCR", &mut f)?;

    // Export NSCR to image
    let ref mut f_w = File::create("test_files/rocker_bg.png")?;
    let scr = nuclear::img::nscr::NSCR::from_ndsfile(&nds)?;
    nuclear::img::renderer::Renderer.export_tilemap(f_w, &clr, &cgr, &scr)?;

    // Re-export NSCR file
    let nds = scr.to_ndsfile("rocker_bg.NSCR".to_string(), ByteOrder::LittleEndian)?;
    let mut f_w = File::create("test_files/rocker_bg.out.NSCR")?;
    nds.to_file(&mut f_w)?;

    Ok(())
}
*/
