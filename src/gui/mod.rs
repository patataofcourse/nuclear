use eframe::egui::{
    containers::Frame, menu, CentralPanel, Color32, Context, SidePanel, Style, TopBottomPanel,
};

pub enum EditorType {
    Palette,
    Tileset,
    Tilemap,
    Frame,
    Animation,
}

impl ToString for EditorType {
    //TODO: localization
    fn to_string(&self) -> String {
        match self {
            Palette => String::from("Palette"),
            Tileset => String::from("Tileset"),
            Tilemap => String::from("Tilemap"),
            Frame => String::from("Frame"),
            Animation => String::from("Animation"),
        }
    }
}

pub struct NuclearApp {
    pub tabs: Vec<(String, EditorType)>,
}

impl Default for NuclearApp {
    fn default() -> Self {
        Self { tabs: vec![] }
    }
}

pub fn menu_bar(ctx: &Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                ui.button("Open");
                ui.menu_button("Test", |ui| {
                    ui.label("gad");
                });
            });
        });
    });
}

pub fn side_panel(ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
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
}

//TODO: separate into functions, move to lib::gui
impl eframe::App for NuclearApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        menu_bar(ctx);

        side_panel(ctx);

        //Main workspace
        CentralPanel::default().show(ctx, |ui| {
            // File selector
            ui.horizontal(|ui| {
                Frame::group(&Style::default())
                    .fill(Color32::LIGHT_GRAY)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.visuals_mut();
                            ui.label("rocker (Frames)");
                            ui.button("X");
                        });
                    });
                ui.separator();
                Frame::group(&Style::default()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.visuals_mut();
                        ui.label("rocker (Animation)");
                        ui.button("X");
                    });
                });
                ui.separator();
                Frame::group(&Style::default()).show(ui, |ui| {
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
