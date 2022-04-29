use bytestream::ByteOrder;
use std::fs::File;

fn main() -> nuclear::error::Result<()> {
    // Open NCLR file
    let mut f = File::open("test_files/rocker.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCLR", &mut f)?;

    // Export NCLR to palette set
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(&nds)?;
    clr.to_dir("test_files/rocker_pal".into())?;

    // Re-export NCLR file
    let nds = clr.to_ndsfile("rocker.NCLR".to_string(), ByteOrder::LittleEndian)?;
    let mut f_w = File::create("test_files/rocker.out.NCLR")?;
    nds.to_file(&mut f_w)?;

    let mut f = File::open("test_files/rocker.NCBR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCBR", &mut f)?;

    let ref mut f_w = File::create("test_files/rocker.tiles.png")?;
    let cgr = nuclear::img::ncgr::NCGR::from_ndsfile(&nds)?;
    nuclear::img::renderer::Renderer.export_tilesheet(f_w, &clr.palettes[&3], &cgr, 8, true)?;
    Ok(())
}
