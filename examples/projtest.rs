use bytestream::ByteOrder;
use std::{fs::File, path::PathBuf};

fn main() -> nuclear::error::Result<()> {
    let mut proj = nuclear::proj::NuclearProject::new(
        "the super cool project",
        "patataofcourse",
        "",
        PathBuf::from("test_files/rockers"),
    )?;

    let mut f = File::open("test_files/rocker_bg.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCLR", &mut f)?;
    let clr = nuclear::img::NCLR::from_ndsfile(&nds)?;

    let mut f = File::open("test_files/rocker_bg.NCGR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NCGR", &mut f)?;
    let cgr = nuclear::img::NCGR::from_ndsfile(&nds)?;

    let mut f = File::open("test_files/rocker_bg.NSCR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker_bg.NSCR", &mut f)?;
    let scr = nuclear::img::NSCR::from_ndsfile(&nds)?;

    proj.insert_nclr("rocker_bg", &clr)?;
    proj.insert_ncgr("rocker_bg", &cgr)?;
    proj.insert_nscr("rocker_bg", &scr)?;

    let mut f = File::open("test_files/rocker.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCLR", &mut f)?;
    let clr = nuclear::img::NCLR::from_ndsfile(&nds)?;

    let mut f = File::open("test_files/rocker.NCBR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("rocker.NCBR", &mut f)?;
    let cgr = nuclear::img::NCGR::from_ndsfile(&nds)?;

    proj.insert_nclr("rocker", &clr)?;
    proj.insert_ncgr("rocker", &cgr)?;

    let _nds = proj
        .get_nscr("rocker_bg")?
        .unwrap()
        .to_ndsfile("rocker_bg.NSCR".to_string(), ByteOrder::LittleEndian)?;

    Ok(())
}
