use super::message;
use crate::img::NCLR;
use eframe::egui::{containers::Frame, Ui};

#[derive(Clone, Debug)]
pub enum Editor {
    Palette {
        transparency: bool,
        contents: NCLR,
    },
    Tileset {},
    Tilemap {},
    Frames {},
    Animation {},
    Metadata {
        proj_creation: bool,
        name: String,
        author: String,
        description: String,
    },
}

impl Editor {
    pub const fn editor_type(&self) -> &'static str {
        match self {
            Self::Palette { .. } => "Palette",
            Self::Tileset { .. } => "Tileset",
            Self::Tilemap { .. } => "Tilemap",
            Self::Frames { .. } => "Frames",
            Self::Animation { .. } => "Animation",
            Self::Metadata { .. } => "Project metadata",
        }
    }
    pub fn palette(contents: NCLR) -> Self {
        Self::Palette {
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
            ui.heading(format!("{} editor", self.editor_type()));
            match self {
                Self::Palette {
                    transparency,
                    contents,
                } => {
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
                Self::Tileset {} => {
                    ui.label("Not implemented");
                }
                Self::Tilemap {} => {
                    ui.label("Not implemented");
                }
                Self::Frames {} => {
                    ui.label("Not implemented");
                }
                Self::Animation {} => {
                    ui.label("Not implemented");
                }
                Self::Metadata {
                    proj_creation,
                    name,
                    author,
                    description,
                } => {
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
