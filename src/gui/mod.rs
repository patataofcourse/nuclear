use crate::proj::NuclearProject;
use eframe::egui::{CentralPanel, Context, ScrollArea, SidePanel, Ui};

pub mod editor;
pub mod menu_bar;
pub mod message;
pub mod tab;

use self::{
    editor::Editor,
    menu_bar::MenuBarResponse,
    tab::{Tab, TabBarResponse},
};

pub struct NuclearApp {
    pub project: Option<NuclearProject>,
    pub tabs: Vec<(String, Editor)>,
    pub selected_tab: usize,
}

impl NuclearApp {
    pub fn test() -> Self {
        Self {
            tabs: vec![
                ("rocker".to_string(), Editor::palette()),
                ("rocker".to_string(), Editor::Tileset {}),
                ("rocker".to_string(), Editor::Tilemap {}),
                ("rocker".to_string(), Editor::Frames {}),
                ("rocker".to_string(), Editor::Animation {}),
            ],
            selected_tab: 0,
            project: None,
        }
    }
}

impl Default for NuclearApp {
    fn default() -> Self {
        Self {
            tabs: vec![],
            selected_tab: 0,
            project: None,
        }
    }
}

pub fn side_panel(ctx: &Context) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Project - Rockers");
        ui.collapsing("Palettes", |ui| {
            ui.link("rocker_bg");
            ui.link("rocker");
        });
        ui.collapsing("Tilesets", |ui| {
            ui.link("rocker_bg");
            ui.link("rocker");
        });
        ui.collapsing("Tilemaps", |ui| {
            ui.link("rocker_bg");
        });
        ui.collapsing("Animation frames", |ui| {
            ui.link("rocker");
        });
        ui.collapsing("Animations", |ui| {
            ui.link("rocker");
        });
    });
}

pub fn tab_bar(tabs: &Vec<(String, Editor)>, ui: &mut Ui, selected_tab: usize) -> TabBarResponse {
    let mut out = TabBarResponse::None;

    ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut c = 0;
            for tab in tabs {
                let response = ui.add(Tab {
                    name: tab.0.as_str(),
                    editor_type: tab.1.editor_type(),
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
    });

    out
}

impl eframe::App for NuclearApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match menu_bar::menu_bar(ctx) {
            MenuBarResponse::NewProj => message::info("thing", "make new project here"),
            MenuBarResponse::None => {}
        }

        side_panel(ctx);

        CentralPanel::default().show(ctx, |ui| {
            if let None = self.project {
                ui.heading("No project open!");
                ui.label("Use File > New to start a new project or File > Open to open one");
            } else if self.tabs.len() == 0 {
                ui.heading("No files open!");
                ui.label("Click one of the files on the sidebar to open it on the editor");
            } else {
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
                if self.tabs.len() != 0 {
                    self.tabs[self.selected_tab].1.draw(ui);
                }
            }
        });
    }
}
