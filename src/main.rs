use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut f = File::open("test_files/pause.NCLR")?;
    let a = nuclear::ndsfile::NDSFile::from_file(&mut f)?;
    println!("{:?}", a);
    Ok(())
}
