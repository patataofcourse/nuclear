#![allow(dead_code)]

use std::path::{Path, PathBuf};
use tinyfiledialogs::{MessageBoxIcon, YesNo};

fn sanitize_string(input: &str) -> String {
    input
        .replace('`', "\\`")
        .replace('\'', "‘")
        .replace('"', "“")
}

pub fn yes_no(title: &str, contents: &str) -> bool {
    let title = sanitize_string(title);
    let contents = sanitize_string(contents);
    tinyfiledialogs::message_box_yes_no(&title, &contents, MessageBoxIcon::Question, YesNo::No)
        == YesNo::Yes
}

pub fn info(title: &str, contents: &str) {
    let title = sanitize_string(title);
    let contents = sanitize_string(contents);
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Info);
}
pub fn warning(title: &str, contents: &str) {
    let title = sanitize_string(title);
    let contents = sanitize_string(contents);
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Warning);
}
pub fn error(title: &str, contents: &str) {
    let title = sanitize_string(title);
    let contents = sanitize_string(contents);
    tinyfiledialogs::message_box_ok(&title, &contents, MessageBoxIcon::Error);
}

pub fn open_files(
    title: &str,
    path: &Path,
    filter: Option<(&[&str], &str)>,
) -> Option<Vec<PathBuf>> {
    let title = sanitize_string(title);
    tinyfiledialogs::open_file_dialog_multi(&title, path.as_os_str().to_str()?, filter).map(|c| {
        let mut out = vec![];
        for i in c {
            out.push(i.into())
        }
        out
    })
}

pub fn save_file(title: &str, path: &Path) -> Option<PathBuf> {
    let title = sanitize_string(title);
    tinyfiledialogs::save_file_dialog(&title, path.as_os_str().to_str()?).map(|c| c.into())
}

pub fn open_folder(title: &str, path: &Path) -> Option<PathBuf> {
    let title = sanitize_string(title);
    tinyfiledialogs::select_folder_dialog(&title, path.as_os_str().to_str()?).map(|c| c.into())
}
