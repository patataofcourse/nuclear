use std::fs::File;

fn main() -> nuclear::error::Result<()> {
    // Open NCLR file
    let mut f = File::open("test_files/pause.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("pause.NCLR", &mut f)?;

    // Export NCLR to palette set
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(&nds)?;
    clr.to_dir("test_files/pause".into())?;

    // Re-export NCLR file
    let mut f_w = File::create("test_files/pause.out.NCLR")?;
    nds.to_file(&mut f_w)?;

    Ok(())
}
