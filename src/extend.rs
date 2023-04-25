#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Nintendo,
}

impl FileType {
    pub fn filters(&self) -> Option<(&[&str], &str)> {
        match self {
            FileType::Nintendo => Some((
                &[
                    "*.nclr", "*.ncgr", "*.ncbr", "*.nscr", "*.NCLR", "*.NCGR", "*.NCBR", "*.NSCR",
                ],
                ("Nintendo DS image files"),
            )),
        }
    }
}
