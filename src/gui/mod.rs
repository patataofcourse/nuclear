use eframe::egui::{menu, widgets, CentralPanel, Context, Layout, SidePanel, TopBottomPanel, Ui};

pub mod editor;

use self::editor::{
    tab::{Tab, TabBarResponse},
    EditorType,
};

pub struct NuclearApp {
    pub tabs: Vec<(String, EditorType)>,
    pub selected_tab: usize,
}

impl NuclearApp {
    pub fn test() -> Self {
        Self {
            tabs: vec![
                ("rocker".to_string(), EditorType::Frame),
                ("rocker".to_string(), EditorType::Animation),
                ("rocker".to_string(), EditorType::Tileset),
            ],
            selected_tab: 0,
        }
    }
}

impl Default for NuclearApp {
    fn default() -> Self {
        Self {
            tabs: vec![],
            selected_tab: 0,
        }
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

pub fn tab_bar(
    tabs: &Vec<(String, EditorType)>,
    ui: &mut Ui,
    selected_tab: usize,
) -> TabBarResponse {
    let mut out = TabBarResponse::None;

    ui.horizontal(|ui| {
        let mut c = 0;
        for tab in tabs {
            let response = ui.add(Tab {
                name: tab.0.as_str(),
                editor_type: tab.1,
                selected: c == selected_tab,
            });

            if response.changed() {
                out = TabBarResponse::Close(c);
            } else if response.clicked() {
                out = TabBarResponse::Select(c);
            }

            if c != tabs.len() - 1 {
                ui.separator();
            }
            c += 1;
        }
    });

    out
}

impl eframe::App for NuclearApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        menu_bar(ctx);

        side_panel(ctx);

        //Main workspace
        CentralPanel::default().show(ctx, |ui| {
            match tab_bar(&self.tabs, ui, self.selected_tab) {
                TabBarResponse::Select(c) => {
                    self.selected_tab = c;
                }
                TabBarResponse::Close(c) => {
                    if self.selected_tab >= c && self.selected_tab != 0 {
                        self.selected_tab -= 1;
                    }
                    self.tabs.remove(c);
                }
                _ => {}
            }
            ui.separator();

            // Actual workspace
            ui.heading("me when i nuclear");
            ui.label("Hello world!");
            ui.button("here's a useless button");
            ui.label("fuck");
        });
    }
}
