use crate::proj::NuclearProject;
use eframe::egui::{CentralPanel, Context, ScrollArea, SidePanel, Ui};
use std::panic::PanicInfo;

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

    pub fn close_project(&mut self) -> bool {
        //TODO: check if unsaved

        self.project = None;
        self.tabs = vec![];
        self.selected_tab = 0;
        return true;
    }

    pub fn create_project(&mut self) {
        todo!();
    }

    pub fn open_project(&mut self) {
        todo!();
    }

    pub fn save_project(&mut self) {
        todo!();
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
            MenuBarResponse::NewProj => {
                if self.close_project() {
                    self.tabs.push((
                        String::new(),
                        Editor::Metadata {
                            proj_creation: true,
                            name: String::new(),
                            author: String::new(),
                            description: String::new(),
                        },
                    ))
                }
            }
            MenuBarResponse::None => {}
        }

        side_panel(ctx);

        CentralPanel::default().show(ctx, |ui| {
            if self.tabs.len() == 0 {
                if let None = self.project {
                    ui.heading("No project open!");
                    ui.label("Use File > New to start a new project or File > Open to open one");
                } else {
                    ui.heading("No files open!");
                    ui.label("Click one of the files on the sidebar to open it on the editor");
                }
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

pub fn panic_hook(info: &PanicInfo) {
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
