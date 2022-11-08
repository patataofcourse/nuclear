use super::message;
use crate::img::NCLR;
use eframe::egui::{containers::Frame, Ui};

#[derive(Clone, Debug)]
pub enum Editor {
    Palette {
        name: String,
        transparency: bool,
        contents: NCLR,
    },
    Tileset {
        name: String,
    },
    Tilemap {
        name: String,
    },
    Frames {
        name: String,
    },
    Animation {
        name: String,
    },
    Metadata {
        proj_creation: bool,
        name: String,
        author: String,
        description: String,
    },
}

impl Editor {
    pub fn tab_name(&self) -> String {
        match self {
            Self::Palette { name, .. } => format!("{} (Palette)", name),
            Self::Tileset { name, .. } => format!("{} (Tileset)", name),
            Self::Tilemap { name, .. } => format!("{} (Tilemap)", name),
            Self::Frames { name, .. } => format!("{} (Frames)", name),
            Self::Animation { name, .. } => format!("{} (Animation)", name),
            Self::Metadata { .. } => "Project metadata".to_string(),
        }
    }
    pub fn palette(name: String, contents: NCLR) -> Self {
        Self::Palette {
            name,
            transparency: false,
            contents,
        }
    }
}

pub enum EditorResponse {
    None,
    Metadata(MetadataResponse),
}

pub enum MetadataResponse {
    CreateProj,
    Save,
}

impl Editor {
    #[must_use]
    pub fn draw(&mut self, ui: &mut Ui) -> EditorResponse {
        let mut response = EditorResponse::None;
        ui.vertical(|ui| {
            match self {
                Self::Palette {
                    transparency,
                    contents,
                    ..
                } => {
                    ui.heading("Palette editor");
                    ui.horizontal(|ui| {
                        Frame::group(ui.style()).show(ui, |ui| {
                            ui.set_width(200.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label(format!(
                                    "Transparency: {}",
                                    if *transparency { "on" } else { "off" }
                                ));
                                ui.label("Palette 0");
                                ui.label("Palette 1");
                                //TODO
                            })
                        });
                        ui.vertical(|ui| {
                            ui.checkbox(transparency, "Enable transparency");
                            ui.button("Import .pal file");
                            ui.button("Export .pal file");
                        })
                    });
                    ui.button("Save");
                }
                Self::Tileset { .. } => {
                    ui.heading("Tileset editor");
                    ui.label("Not implemented");
                }
                Self::Tilemap { .. } => {
                    ui.heading("Tilemap editor");
                    ui.label("Not implemented");
                }
                Self::Frames { .. } => {
                    ui.heading("Frame editor");
                    ui.label("Not implemented");
                }
                Self::Animation { .. } => {
                    ui.heading("Animation editor");
                    ui.label("Not implemented");
                }
                Self::Metadata {
                    proj_creation,
                    name,
                    author,
                    description,
                } => {
                    ui.heading("Project metadata settings");
                    response = if let Some(r) =
                        Self::draw_metadata(ui, proj_creation, name, author, description)
                    {
                        EditorResponse::Metadata(r)
                    } else {
                        EditorResponse::None
                    }
                }
            }
        });
        response
    }

    fn draw_metadata(
        ui: &mut Ui,
        proj_creation: &mut bool,
        name: &mut String,
        author: &mut String,
        description: &mut String,
    ) -> Option<MetadataResponse> {
        {
            if *proj_creation {
                ui.label("Fill in the following parameters to create your project:\n");
            }

            ui.horizontal(|ui| {
                ui.label("Project name (required): ");
                ui.text_edit_singleline(name);
            });

            ui.horizontal(|ui| {
                ui.label("Author(s) (required): ");
                ui.text_edit_singleline(author);
            });

            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_multiline(description);
            });

            ui.label("");

            if ui
                .button(if *proj_creation {
                    "Create project"
                } else {
                    "Save"
                })
                .clicked()
            {
                if name == "" || author == "" {
                    message::error("Metadata incomplete", "Project name and author required");
                    return None;
                }
                Some(if *proj_creation {
                    MetadataResponse::CreateProj
                } else {
                    MetadataResponse::Save
                })
            } else {
                None
            }
        }
    }
}
