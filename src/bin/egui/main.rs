use std::panic::{self, PanicInfo};

/// Actual GUI implementation
pub mod gui;
/// This module is supposed to offer abstraction to whatever dialog crate nuclear uses
/// just in case I have to switch to another crate (or implement my own dialogs)
pub mod message;

fn main() {
    panic::set_hook(Box::new(panic_hook));

    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light; //TODO: settings
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(gui::NuclearApp::default())),
    );
}

pub fn panic_hook(info: &PanicInfo) {
    let location = info.location();
    let payload = info.payload();
    let payload_text = if let Some(s) = payload.downcast_ref::<&str>() {
        format!("More details: {}", s)
    } else if let Some(s) = payload.downcast_ref::<String>() {
        format!("More details: {}", s)
    } else {
        "Could not get detailed panic information".to_string()
    };

    let panic_text = format!(
        "Rust panic {}\n{}",
        if let Some(location) = location {
            format!(
                "at {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        } else {
            "".to_string()
        },
        payload_text
    );

    message::error("Error - panic!", &panic_text);
}
