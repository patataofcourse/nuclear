use bytestream::ByteOrder;
use nuclear::ndsfile::NDSFileType;
use std::{fs::File, path::PathBuf};

fn main() -> nuclear::error::Result<()> {
    let mut proj = nuclear::proj::NuclearProject::new(
        "the super cool project",
        "patataofcourse",
        "",
        PathBuf::from("test_files/rockers"),
    )?;

    let mut f = File::open("test_files/rocker_bg.NCLR")?;
    let clr = nuclear::img::NCLR::from_file("rocker_bg.NCLR", &mut f)?;

    let mut f = File::open("test_files/rocker_bg.NCGR")?;
    let cgr = nuclear::img::NCGR::from_file("rocker_bg.NCGR", &mut f)?;

    let mut f = File::open("test_files/rocker_bg.NSCR")?;
    let scr = nuclear::img::NSCR::from_file("rocker_bg.NSCR", &mut f)?;

    proj.insert_nclr("rocker_bg", &clr)?;
    proj.insert_ncgr("rocker_bg", &cgr)?;
    proj.insert_nscr("rocker_bg", &scr)?;

    let mut f = File::open("test_files/rocker.NCLR")?;
    let clr = nuclear::img::NCLR::from_file("rocker.NCLR", &mut f)?;

    let mut f = File::open("test_files/rocker.NCBR")?;
    let cgr = nuclear::img::NCGR::from_file("rocker.NCBR", &mut f)?;

    proj.insert_nclr("rocker", &clr)?;
    proj.insert_ncgr("rocker", &cgr)?;

    let _nds = proj
        .get_nscr("rocker_bg")?
        .unwrap()
        .to_ndsfile("rocker_bg.NSCR".to_string(), ByteOrder::LittleEndian)?;

    Ok(())
}
