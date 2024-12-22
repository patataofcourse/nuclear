use std::{fs::File, io::Read};

use nuclear::{img::ncer::NCER, ndsfile::NDSFileType};

pub fn main() -> nuclear::Result<()> {
    let mut ncer = <NCER as NDSFileType>::from_file(
        "test_files/rocker.NCER",
        &mut File::open("test_files/rocker.NCER")?,
    )?;

    println!("{:?}", ncer);

    Ok(())
}
