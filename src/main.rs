use std::fs::File;

fn main() -> nuclear::error::Result<()> {
    let mut f = File::open("test_files/pause.NCLR")?;
    let nds = nuclear::ndsfile::NDSFile::from_file("pause.NCLR", &mut f)?;
    let clr = nuclear::img::nclr::NCLR::from_ndsfile(nds)?;
    clr.to_dir("test_files/pause".into())?;
    Ok(())
}
