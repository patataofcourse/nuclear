use bytestream::ByteOrder;
use nuclear::ndsfile::NDSFileType;
use std::fs::File;

const FOLDER_NAME: &str = "ver2";
const NAME: &str = "rocker";
const TILES_EXTENSION: &str = "NCBR";
const NSCR: bool = false;

fn main() -> nuclear::error::Result<()> {
    // Open NCLR file
    let mut f = File::open(format!("test_files/{}/{}.NCLR", FOLDER_NAME, NAME))?;
    let nds = nuclear::ndsfile::NDSFile::from_file(&format!("{}.NCLR", NAME), &mut f)?;

    // Export NCLR to palette set
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(&nds)?;
    nuclear::img::export::export_palettes(
        &clr,
        format!("test_files/out/{}/{}_pal", FOLDER_NAME, NAME).into(),
    )?;

    // Re-export NCLR file
    let nds = clr.to_ndsfile(format!("{}.NCLR", NAME), ByteOrder::LittleEndian)?;
    let mut f_w = File::create(format!("test_files/out/{}/{}.NCLR", FOLDER_NAME, NAME))?;
    nds.to_file(&mut f_w)?;

    // Open NCGR/NCBR file
    let mut f = File::open(format!(
        "test_files/{}/{}.{}",
        FOLDER_NAME, NAME, TILES_EXTENSION
    ))?;
    let nds =
        nuclear::ndsfile::NDSFile::from_file(&format!("{}.{}", NAME, TILES_EXTENSION), &mut f)?;

    // Export NCGR to tilesheet
    let f_w = &mut File::create(format!("test_files/out/{}/{}.tiles.png", FOLDER_NAME, NAME))?;
    let cgr = nuclear::img::ncgr::NCGR::from_ndsfile(&nds)?;
    nuclear::img::export::export_tilesheet(f_w, &clr.palettes[&0], &cgr, 32, false)?;

    // Re-export NCGR file
    let nds = cgr.to_ndsfile(
        format!("{}.{}", NAME, TILES_EXTENSION),
        ByteOrder::LittleEndian,
    )?;
    let mut f_w = File::create(format!(
        "test_files/out/{}/{}.{}",
        FOLDER_NAME, NAME, TILES_EXTENSION
    ))?;
    nds.to_file(&mut f_w)?;

    if NSCR {
        // Open NSCR file
        let mut f = File::open(format!("test_files/{}/{}.NSCR", FOLDER_NAME, NAME))?;
        let nds = nuclear::ndsfile::NDSFile::from_file(&format!("{}.NSCR", NAME), &mut f)?;
        // Export NSCR to image
        let f_w = &mut File::create(format!("test_files/out/{}/{}.png", FOLDER_NAME, NAME))?;
        let scr = nuclear::img::nscr::NSCR::from_ndsfile(&nds)?;
        nuclear::img::export::export_tilemap(f_w, &clr, &cgr, &scr)?;
        // Re-export NSCR file
        let nds = scr.to_ndsfile(format!("{}.NSCR", NAME), ByteOrder::LittleEndian)?;
        let mut f_w = File::create(format!("test_files/out/{}/{}.NSCR", FOLDER_NAME, NAME))?;
        nds.to_file(&mut f_w)?;
    }

    Ok(())
}
