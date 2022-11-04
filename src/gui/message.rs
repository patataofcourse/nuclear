use native_dialog::{MessageDialog, MessageType};

pub fn info(title: &str, contents: &str) {
    match MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(contents)
        .show_alert()
    {
        Ok(_) => {}
        Err(_) => println!("INFO: {}\n{}\n\n", title, contents),
    };
}

pub fn confirm(title: &str, contents: &str) -> bool {
    match MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(contents)
        .show_confirm()
    {
        Ok(c) => c,
        Err(_) => panic!("Cannot open a confirmation prompt"),
    }
}

pub fn warning(title: &str, contents: &str) {
    match MessageDialog::new()
        .set_type(MessageType::Warning)
        .set_title(title)
        .set_text(contents)
        .show_alert()
    {
        Ok(_) => {}
        Err(_) => println!("WARNING: {}\n{}\n\n", title, contents),
    };
}
pub fn error(title: &str, contents: &str) {
    match MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(title)
        .set_text(contents)
        .show_alert()
    {
        Ok(_) => {}
        Err(_) => println!("ERROR: {}\n{}\n\n", title, contents),
    };
}

pub fn file_picker() {
    todo!();
}
