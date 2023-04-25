#![allow(dead_code)]

use std::path::{Path, PathBuf};
use tinyfiledialogs::{MessageBoxIcon, YesNo};

pub fn yes_no(title: &str, contents: &str) -> bool {
    let title = title.replace('`', "\\`");
    let contents = contents.replace('`', "\\`");
    tinyfiledialogs::message_box_yes_no(&title, &contents, MessageBoxIcon::Question, YesNo::No)
        == YesNo::Yes
}

pub fn info(title: &str, contents: &str) {
    let title = title.replace('`', "\\`");
    let contents = contents.replace('`', "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Info);
}
pub fn warning(title: &str, contents: &str) {
    let title = title.replace('`', "\\`");
    let contents = contents.replace('`', "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Warning);
}
pub fn error(title: &str, contents: &str) {
    let title = title.replace('`', "\\`");
    let contents = contents.replace('`', "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Error);
}

pub fn open_file(title: &str, path: &Path, filter: Option<(&[&str], &str)>) -> Option<PathBuf> {
    let title = title.replace('`', "\\`");
    tinyfiledialogs::open_file_dialog(&title, path.as_os_str().to_str()?, filter).map(|c| c.into())
}

pub fn save_file(title: &str, path: &Path) -> Option<PathBuf> {
    let title = title.replace('`', "\\`");
    tinyfiledialogs::save_file_dialog(&title, path.as_os_str().to_str()?).map(|c| c.into())
}

pub fn open_folder(title: &str, path: &Path) -> Option<PathBuf> {
    let title = title.replace('`', "\\`");
    tinyfiledialogs::select_folder_dialog(&title, path.as_os_str().to_str()?).map(|c| c.into())
}
