use std::{fs::File, path::Path};

use crate::{addon::NuclearResult, message, widgets::tab::Tab};
use eframe::egui::{CentralPanel, Context, RichText, ScrollArea, SidePanel, Ui};
use nuclear::{img::export, proj::NuclearProject};

pub mod editor;
pub mod menu_bar;

use self::{
    editor::{Editor, EditorResponse},
    menu_bar::MenuBarResponse,
};

#[derive(Default)]
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
        true
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
                    if project.palette_sets.is_empty() {
                        ui.label("None");
                    }
                    for (name, set) in &project.palette_sets {
                        if ui.link(name).clicked() {
                            let mut already_open = None;
                            for i in 0..app.editors.len() {
                                let editor = &app.editors[i];
                                if let Editor::Palette { name: name_, .. } = editor {
                                    if name == name_ {
                                        already_open = Some(i);
                                        break;
                                    }
                                }
                            }

                            if let Some(i) = already_open {
                                app.selected_tab = i;
                            } else {
                                app.editors
                                    .push(Editor::palette(name.clone(), set.get_inner().manage()));
                                app.selected_tab = app.editors.len() - 1;
                            }
                        }
                    }
                });
                ui.collapsing("Tilesets", |ui| {
                    if project.tilesets.is_empty() {
                        ui.label("None");
                    }
                    for (name, set) in &project.tilesets {
                        if ui.link(name).clicked() {
                            let mut already_open = None;
                            for i in 0..app.editors.len() {
                                let editor = &app.editors[i];
                                if let Editor::Tileset { name: name_, .. } = editor {
                                    if name == name_ {
                                        already_open = Some(i);
                                        break;
                                    }
                                }
                            }

                            if let Some(i) = already_open {
                                app.selected_tab = i;
                            } else {
                                app.editors.push(Editor::tileset(
                                    name.clone(),
                                    set.get_inner().manage(),
                                    set.associated_palette.clone(),
                                ));
                                app.selected_tab = app.editors.len() - 1;
                            }
                        }
                    }
                });
                ui.collapsing("Tilemaps", |ui| {
                    if project.tilemaps.is_empty() {
                        ui.label("None");
                    }
                    for (name, map) in &project.tilemaps {
                        if ui.link(name).clicked() {
                            let mut already_open = None;
                            for i in 0..app.editors.len() {
                                let editor = &app.editors[i];
                                if let Editor::Tilemap { name: name_, .. } = editor {
                                    if name == name_ {
                                        already_open = Some(i);
                                        break;
                                    }
                                }
                            }

                            if let Some(i) = already_open {
                                app.selected_tab = i;
                            } else {
                                app.editors.push(Editor::tilemap(
                                    name.clone(),
                                    map.get_inner().manage(),
                                    map.associated_tileset.clone(),
                                ));
                                app.selected_tab = app.editors.len() - 1;
                            }
                        }
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

    ui.push_id("tab_bar", |ui| {
        ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for (c, editor) in editors.iter().enumerate() {
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
                }
            });
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
                if let Some(path) = message::open_folder("Open project folder", Path::new("")) {
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
            MenuBarResponse::ImportFile(c) => {
                if let Some(files) =
                    message::open_files("Open Nintendo file", Path::new(""), c.filters())
                {
                    for file in files {
                        todo!("identify files, add to project, ask for name of files?");
                    }
                }
            }
            MenuBarResponse::None => {}
        }

        side_panel(ctx, self);

        CentralPanel::default().show(ctx, |ui| {
            if self.editors.is_empty() {
                if self.project.is_none() {
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

                if !self.editors.is_empty() {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        match self.editors[self.selected_tab].draw(self.project.as_ref().unwrap(), ui) {
                            EditorResponse::CreateProj => {
                                let Editor::Metadata { name, author, description, ..} =  &self.editors[self.selected_tab] else {
                                    unreachable!();
                                };

                                let (name, author, description) = (name.to_string(), author.to_string(), description.to_string());

                                if let Some(path) = message::open_folder("Choose empty folder for new project", Path::new("")) {
                                    message::info(
                                        "Project created!",
                                        &format!("Successfully created project {}", name),
                                    );
                                    self.editors.remove(self.selected_tab);

                                    if self.selected_tab != 0 {
                                        self.selected_tab -= 1;
                                    }

                                    self.project =
                                        Some(NuclearProject::new(&name, &author, &description, path).manage());
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
                                project.save().manage();
                                message::info("Project metadata", "Saved project metadata!");
                            }
                            EditorResponse::SavePalette => {
                                todo!("Save palette event");
                            }
                            EditorResponse::SaveTset => {
                                let Editor::Tileset { name, contents, palette, ..} =  &self.editors[self.selected_tab] else {
                                    unreachable!();
                                };
                                let project = self.project.as_mut().unwrap();
                                project.insert_ncgr(name, contents).manage();

                                //TODO: might need to make an insert_ncgr_with_meta or smth cause this just feels wrong
                                let tileset = project.tilesets.get_mut(name).unwrap();
                                tileset.associated_palette = palette.clone();

                                project.save().manage();

                                message::info("Saved correctly!", &format!("Saved tileset {}.", name))
                            }
                            EditorResponse::SaveTmap => {
                                let Editor::Tilemap { name, contents, tileset, ..} = &self.editors[self.selected_tab] else {
                                    unreachable!();
                                };
                                let project = self.project.as_mut().unwrap();
                                project.insert_nscr(name, contents).manage();

                                //TODO: might need to make an insert_ncgr_with_meta or smth cause this just feels wrong
                                let tilemap = project.tilemaps.get_mut(name).unwrap();
                                tilemap.associated_tileset = tileset.clone();

                                project.save().manage();

                                message::info("Saved correctly!", &format!("Saved tilemap {}.", name))
                            }
                            EditorResponse::ExportPng =>  {
                                let Editor::Tilemap { contents, tileset, tileset_cache, ..} = &mut self.editors[self.selected_tab] else {
                                    unreachable!();
                                };

                                if let Some(c) = tileset {
                                    if let Some(path) = message::save_file("Choose path for exported PNG", Path::new("")) {
                                        if let Some(pixels) =
                                            Editor::render_tilemap_img(contents, self.project.as_ref().unwrap(), c, tileset_cache) {
                                            export::export_image(
                                                &mut File::create(path).manage(),
                                                &pixels,
                                                contents.width as u32,
                                                contents.height as u32,
                                                png::ColorType::Rgba,
                                            ).manage();
                                            message::info("Exported PNG correctly!", "Tilemap successfully exported")
                                        } else {
                                            message::error("Can't export image", "Failed to render image for some reason, are you sure it's valid?");
                                        }
                                    }
                                } else {
                                    message::error("Can't export image", "Tilemap needs an associated tileset to be exported!");
                                }
                            }
                            EditorResponse::None => {}
                        }
                    });
                }
            }
        });
    }
}
