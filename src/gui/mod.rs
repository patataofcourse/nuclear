use eframe::egui::{menu, widgets, CentralPanel, Context, Layout, SidePanel, TopBottomPanel, Ui};

pub mod editor;

use self::editor::EditorType;

pub struct NuclearApp {
    pub tabs: Vec<(String, EditorType)>,
}

impl NuclearApp {
    pub fn test() -> Self {
        Self {
            tabs: vec![
                ("rocker".to_string(), EditorType::Frame),
                ("rocker".to_string(), EditorType::Animation),
                ("rocker".to_string(), EditorType::Tileset),
            ],
        }
    }
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
            ui.button("button!!!");
            ui.with_layout(Layout::right_to_left(), |ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.label("Toggle dark mode");
            });
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
        let mut c = 0;
        for tab in tabs {
            editor::render_tab(ui, tab, c == 0); //todo: use response
            if c != tabs.len() - 1 {
                ui.separator();
            }
            c += 1;
        }
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
            ui.horizontal(|ui| {
                ui.add(editor::tab::Tab {
                    name: "a",
                    editor_type: EditorType::Frame,
                    selected: true,
                });
                ui.add(editor::tab::Tab {
                    name: "b",
                    editor_type: EditorType::Frame,
                    selected: false,
                });
            });
        });
    }
}
