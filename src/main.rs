use nuclear::gui::{message, NuclearApp};
use std::panic::{self, PanicInfo};

fn main() {
    panic::set_hook(Box::new(panic_hook));

    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light; //TODO: settings
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(NuclearApp::test())),
    );
}

fn panic_hook(info: &PanicInfo) {
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
