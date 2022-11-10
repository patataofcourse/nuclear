use crate::{message, widgets::tab::Tab};
use eframe::egui::{CentralPanel, Context, RichText, ScrollArea, SidePanel, Ui};
use nuclear::{error::Error, proj::NuclearProject};

pub mod addon;
pub mod editor;
pub mod menu_bar;

use self::{
    addon::NuclearResult,
    editor::{Editor, EditorResponse},
    menu_bar::MenuBarResponse,
};

pub struct NuclearApp {
    pub project: Option<NuclearProject>,
    pub editors: Vec<Editor>,
    pub selected_tab: usize,
}

impl NuclearApp {
    pub fn close_project(&mut self) -> bool {
        //TODO: check if unsaved

        self.project = None;
        self.editors = vec![];
        self.selected_tab = 0;
        return true;
    }

    /// Saves the contents of all opened editors
    pub fn save_project(&mut self) {
        todo!();
    }
}

impl Default for NuclearApp {
    fn default() -> Self {
        Self {
            editors: vec![],
            selected_tab: 0,
            project: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabBarResponse {
    None,
    Select(usize),
    Close(usize),
}

pub fn side_panel(ctx: &Context, app: &mut NuclearApp) {
    SidePanel::left("side_panel").show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(project) = &app.project {
                ui.label(RichText::new(format!("Project - {}", project.name)).underline());
                ui.collapsing("Palettes", |ui| {
                    if project.palette_sets.len() == 0 {
                        ui.label("None");
                    }
                    for (name, _) in &project.palette_sets {
                        if ui.link(name).clicked() {
                            //TODO: check if already open
                            //TODO: add method to get directly from wrapper
                            app.editors.push(Editor::palette(
                                name.clone(),
                                project.get_nclr(name).manage().unwrap(),
                            ));
                            app.selected_tab = app.editors.len() - 1;
                        }
                    }
                });
                ui.collapsing("Tilesets", |ui| {
                    if project.tilesets.len() == 0 {
                        ui.label("None");
                    }
                    for (name, set) in &project.tilesets {
                        if ui.link(name).clicked() {
                            //TODO: check if already open
                            //TODO: add method to get directly from wrapper
                            app.editors.push(Editor::tileset(
                                name.clone(),
                                project.get_ncgr(name).manage().unwrap(),
                                set.associated_palette.clone(),
                            ));
                            app.selected_tab = app.editors.len() - 1;
                        }
                    }
                });
                ui.collapsing("Tilemaps", |ui| {
                    if project.tilemaps.len() == 0 {
                        ui.label("None");
                    }
                    for (name, map) in &project.tilemaps {
                        ui.link(name);
                    }
                });
                ui.collapsing("Animation frames", |ui| {
                    ui.label("Unimplemented");
                });
                ui.collapsing("Animations", |ui| {
                    ui.label("Unimplemented");
                });
            } else {
                ui.label("No project loaded");
            }
        });
    });
}

pub fn tab_bar(editors: &Vec<Editor>, ui: &mut Ui, selected_tab: usize) -> TabBarResponse {
    let mut out = TabBarResponse::None;

    ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            let mut c = 0;
            for editor in editors {
                let response = ui.add(Tab {
                    name: editor.tab_name(),
                    selected: c == selected_tab,
                });

                if response.changed() {
                    out = TabBarResponse::Close(c);
                } else if response.clicked() {
                    out = TabBarResponse::Select(c);
                }

                if c != editors.len() - 1 {
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
        match menu_bar::menu_bar(self, ctx) {
            MenuBarResponse::NewProj => {
                if self.close_project() {
                    self.editors.push(Editor::Metadata {
                        proj_creation: true,
                        name: String::new(),
                        author: String::new(),
                        description: String::new(),
                    })
                }
            }
            MenuBarResponse::OpenProj => {
                if let Some(path) = message::open_folder("Open project folder", &"".into()) {
                    if self.close_project() {
                        match NuclearProject::load_from_file(&path) {
                            Ok(c) => self.project = Some(c),
                            Err(e) => message::error(
                                "Failed to open project",
                                &format!(
                                    "Project at {} could not be opened:\n{}",
                                    path.display(),
                                    e
                                ),
                            ),
                        }
                    }
                }
            }
            MenuBarResponse::Metadata => {
                let proj = self.project.as_ref().unwrap();
                self.editors.push(Editor::Metadata {
                    proj_creation: false,
                    name: proj.name.clone(),
                    author: proj.author.clone(),
                    description: proj.description.clone(),
                });
                self.selected_tab = self.editors.len() - 1;
            }
            MenuBarResponse::None => {}
        }

        side_panel(ctx, self);

        CentralPanel::default().show(ctx, |ui| {
            if self.editors.len() == 0 {
                if let None = self.project {
                    ui.heading("No project open!");
                    ui.label("Use File > New to start a new project or File > Open to open one");
                } else {
                    ui.heading("No files open!");
                    ui.label("Click one of the files on the sidebar to open it on the editor");
                }
            } else {
                match tab_bar(&self.editors, ui, self.selected_tab) {
                    TabBarResponse::Select(c) => {
                        self.selected_tab = c;
                    }
                    TabBarResponse::Close(c) => {
                        if self.selected_tab >= c && self.selected_tab != 0 {
                            self.selected_tab -= 1;
                        }
                        self.editors.remove(c);
                    }
                    _ => {}
                }

                ui.separator();

                if self.editors.len() != 0 {
                    match self.editors[self.selected_tab].draw(self.project.as_ref().unwrap(), ui) {
                        EditorResponse::CreateProj => {
                            let Editor::Metadata { name, author, description, ..} =  &self.editors[self.selected_tab] else {
                                unreachable!();
                            };

                            let (name, author, description) = (name.to_string(), author.to_string(), description.to_string());

                            if let Some(path) = message::open_folder("Choose empty folder for new project", &"".into()) {
                                message::info(
                                    "Project created!",
                                    &format!("Successfully created project {}", name),
                                );
                                self.editors.remove(self.selected_tab);

                                if self.selected_tab != 0 {
                                    self.selected_tab -= 1;
                                }

                                self.project =
                                    Some(NuclearProject::new(&name, &author, &description, path)?);
                            }
                        }
                        EditorResponse::SaveMetadata => {
                            let Editor::Metadata { name, author, description, ..} =  &self.editors[self.selected_tab] else {
                                unreachable!();
                            };
                            let project = self.project.as_mut().unwrap();
                            project.name = name.to_string();
                            project.author = author.to_string();
                            project.description = description.to_string();
                            message::info("Project metadata", "Saved project metadata!");
                        }
                        EditorResponse::SavePalette => {
                            let todo = 0;
                            todo!();
                        }
                        EditorResponse::None => {}
                    }
                }
            }
            Ok::<(), Error>(())
        });
    }
}
