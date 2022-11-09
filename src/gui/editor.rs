use super::{message, widgets::palette::PalPreview};
use crate::img::NCLR;
use eframe::egui::{self, containers::Frame, Painter, Ui};

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
    SavePalette,
    CreateProj,
    SaveMetadata,
}

impl Editor {
    pub fn draw(&mut self, ui: &mut Ui) -> EditorResponse {
        let mut response = EditorResponse::None;
        ui.vertical(|ui| match self {
            Self::Palette {
                transparency,
                contents,
                ..
            } => {
                ui.heading("Palette editor");
                response = Self::draw_palette(ui, contents, transparency);
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
                response = Self::draw_metadata(ui, proj_creation, name, author, description);
            }
        });
        response
    }

    fn transparency(painter: &Painter, pos: egui::Pos2, size: egui::Vec2) {
        painter.rect(
            [pos, pos + size / 2.0].into(),
            0.0,
            egui::Color32::DARK_GRAY,
            egui::Stroke::none(),
        );
        painter.rect(
            [
                pos + (size.x / 2.0, 0.0).into(),
                pos + (size.x, size.y / 2.0).into(),
            ]
            .into(),
            0.0,
            egui::Color32::LIGHT_GRAY,
            egui::Stroke::none(),
        );
        painter.rect(
            [
                pos + (0.0, size.y / 2.0).into(),
                pos + (size.x / 2.0, size.y).into(),
            ]
            .into(),
            0.0,
            egui::Color32::LIGHT_GRAY,
            egui::Stroke::none(),
        );
        painter.rect(
            [
                pos + (size.x / 2.0, size.y / 2.0).into(),
                pos + (size.x, size.y).into(),
            ]
            .into(),
            0.0,
            egui::Color32::DARK_GRAY,
            egui::Stroke::none(),
        );
    }

    fn draw_palette(ui: &mut Ui, contents: &NCLR, transparency: &mut bool) -> EditorResponse {
        ui.horizontal(|ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(350.0);
                ui.set_height(350.0);
                ui.vertical(|ui| {
                    if contents.is_8_bit {
                        ui.add(PalPreview {
                            color_amt: contents.color_amt,
                            palette: &contents.palettes[&0],
                            is_8_bit: contents.is_8_bit,
                        });
                    } else {
                        for (num, pal) in &contents.palettes {
                            ui.horizontal(|ui| {
                                ui.label(format!("Palette {}", num)).rect;
                                ui.add(PalPreview {
                                    color_amt: contents.color_amt,
                                    palette: pal,
                                    is_8_bit: contents.is_8_bit,
                                });
                            });
                        }
                    }
                })
            });
            ui.vertical(|ui| {
                ui.checkbox(transparency, "Enable transparency");
                ui.button("Import .pal file");
                ui.button("Export .pal file");
            })
        });
        if ui.button("Save").clicked() {
            EditorResponse::SavePalette
        } else {
            EditorResponse::None
        }
    }

    fn draw_metadata(
        ui: &mut Ui,
        proj_creation: &mut bool,
        name: &mut String,
        author: &mut String,
        description: &mut String,
    ) -> EditorResponse {
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
                    return EditorResponse::None;
                }
                if *proj_creation {
                    EditorResponse::CreateProj
                } else {
                    EditorResponse::SaveMetadata
                }
            } else {
                EditorResponse::None
            }
        }
    }
}
