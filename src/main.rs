use nuclear::gui::NuclearApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(NuclearApp::default())),
    );
}
