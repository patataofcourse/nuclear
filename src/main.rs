use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "nuclear",
        options,
        Box::new(|_cc| Box::new(NuclearApp::default())),
    );
}

struct NuclearApp;

impl Default for NuclearApp {
    fn default() -> Self {
        Self
    }
}

//TODO: separate into functions, move to lib::gui
impl eframe::App for NuclearApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Toolbar / menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.button("Open");
                });
            });
        });

        //Side panel / project structure
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Project name");
            ui.collapsing("Palettes", |ui| {
                ui.label("rocker_bg");
                ui.label("rocker");
            });
            ui.collapsing("Tilesets", |ui| {
                ui.label("rocker_bg");
                ui.label("rocker");
            });
            ui.collapsing("Tilemaps", |ui| {
                ui.label("rocker_bg");
            });
            ui.collapsing("Animation frames", |ui| {
                ui.label("rocker");
            });
            ui.collapsing("Animations", |ui| {
                ui.label("rocker");
            });
        });

        //Main workspace
        egui::CentralPanel::default().show(ctx, |ui| {
            // File selector
            ui.horizontal(|ui| {
                egui::containers::Frame::group(&egui::Style::default())
                    .fill(egui::Color32::LIGHT_GRAY)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.visuals_mut();
                            ui.label("rocker (Frames)");
                            ui.button("X");
                        });
                    });
                ui.separator();
                egui::containers::Frame::group(&egui::Style::default()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.visuals_mut();
                        ui.label("rocker (Animation)");
                        ui.button("X");
                    });
                });
                ui.separator();
                egui::containers::Frame::group(&egui::Style::default()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.visuals_mut();
                        ui.label("rocker (Tileset)");
                        ui.button("X");
                    });
                });
            });
            ui.separator();

            // Actual workspace
            ui.heading("me when i nuclear");
            ui.horizontal(|ui| {
                ui.label("Here's a button");
                //ui.text_edit_singleline(&mut self.name);
                ui.button("button!!!");
            });
            ui.label("Hello world!");
        });
    }
}
