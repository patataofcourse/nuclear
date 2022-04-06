use std::fs::File;

fn main() -> nuclear::error::Result<()> {
    let mut f = File::open("test_files/pause.NCLR")?;
    let a = nuclear::ndsfile::NDSFile::from_file(&mut f)?;
    println!("{:?}", a);
    Ok(())
}
