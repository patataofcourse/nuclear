use eframe::egui::{
    containers::Frame, menu, CentralPanel, Color32, Context, SidePanel, Style, TopBottomPanel, Ui,
};

pub mod editor;

use self::editor::EditorType;

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
            ui.button("THE BUTTON WILL SURVIVE!!!!!");
        });
    });
}

pub fn side_panel(ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Project - Rockers");
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

pub fn tab_bar(tabs: &Vec<(String, EditorType)>, ui: &mut Ui) {
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
}

//TODO: separate into functions
impl eframe::App for NuclearApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        menu_bar(ctx);

        side_panel(ctx);

        //Main workspace
        CentralPanel::default().show(ctx, |ui| {
            tab_bar(&self.tabs, ui);
            ui.separator();

            // Actual workspace
            ui.heading("me when i nuclear");
            ui.label("Hello world!");
        });
    }
}
