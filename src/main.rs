use nuclear::gui::NuclearApp;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light; //TODO: setting
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(NuclearApp::test())),
    );
}
