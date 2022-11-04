//use native_dialog::{MessageDialog, MessageType};
use tinyfiledialogs::{MessageBoxIcon, YesNo};

pub fn info(title: &str, contents: &str) {
    tinyfiledialogs::message_box_ok(title, contents, MessageBoxIcon::Info);
}

pub fn yes_no(title: &str, contents: &str) -> bool {
    tinyfiledialogs::message_box_yes_no(title, contents, MessageBoxIcon::Question, YesNo::No)
        == YesNo::Yes
}

pub fn warning(title: &str, contents: &str) {
    tinyfiledialogs::message_box_ok(title, contents, MessageBoxIcon::Warning);
}
pub fn error(title: &str, contents: &str) {
    tinyfiledialogs::message_box_ok(title, contents, MessageBoxIcon::Error);
}

pub fn file_picker() -> Option<String> {
    tinyfiledialogs::open_file_dialog("a", "/", None)
}
