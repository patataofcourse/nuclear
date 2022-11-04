use nuclear::NuclearApp;
use std::panic::{self};

fn main() {
    panic::set_hook(Box::new(nuclear::panic_hook));

    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light; //TODO: settings
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(NuclearApp::test())),
    );
}
