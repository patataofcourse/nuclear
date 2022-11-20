#![allow(dead_code)]

use std::path::PathBuf;
use tinyfiledialogs::{MessageBoxIcon, YesNo};

pub fn yes_no(title: &str, contents: &str) -> bool {
    let title = title.replace("`", "\\`");
    let contents = contents.replace("`", "\\`");
    tinyfiledialogs::message_box_yes_no(&title, &contents, MessageBoxIcon::Question, YesNo::No)
        == YesNo::Yes
}

pub fn info(title: &str, contents: &str) {
    let title = title.replace("`", "\\`");
    let contents = contents.replace("`", "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Info);
}
pub fn warning(title: &str, contents: &str) {
    let title = title.replace("`", "\\`");
    let contents = contents.replace("`", "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Warning);
}
pub fn error(title: &str, contents: &str) {
    let title = title.replace("`", "\\`");
    let contents = contents.replace("`", "\\`");
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Error);
}

pub fn open_file(title: &str, path: &PathBuf, filter: Option<(&[&str], &str)>) -> Option<PathBuf> {
    let title = title.replace("`", "\\`");
    if let Some(c) = tinyfiledialogs::open_file_dialog(&title, path.as_os_str().to_str()?, filter) {
        Some(c.into())
    } else {
        None
    }
}

pub fn open_folder(title: &str, path: &PathBuf) -> Option<PathBuf> {
    let title = title.replace("`", "\\`");
    if let Some(c) = tinyfiledialogs::select_folder_dialog(&title, path.as_os_str().to_str()?) {
        Some(c.into())
    } else {
        None
    }
}