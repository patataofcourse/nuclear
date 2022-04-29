use bytestream::ByteOrder;
use std::fs::File;

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
    nuclear::img::renderer::Renderer.export_tilesheet(f_w, &clr.palettes[&3], &cgr, 32, true)?;
    Ok(())
}
