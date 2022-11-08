use super::message;
use crate::img::NCLR;
use eframe::egui::{self, containers::Frame, Ui};

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

    fn draw_palette(ui: &mut Ui, contents: &NCLR, transparency: &mut bool) -> EditorResponse {
        ui.horizontal(|ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(300.0);
                ui.set_height(300.0);
                ui.vertical(|ui| {
                    for (num, pal) in &contents.palettes {
                        let label_rect = ui.label(format!("Palette {}", num)).rect;
                        let mut min: egui::Pos2 = (label_rect.max.x + 5.0, label_rect.min.y).into();
                        let size = label_rect.max.y - label_rect.min.y;
                        let size: egui::Vec2 = (size, size).into();
                        let painter = ui.painter();
                        if contents.is_8_bit {
                            let todo = 0;
                            todo!();
                        } else {
                            for i in 0..contents.color_amt {
                                if i == 0 && *transparency {
                                    painter.rect(
                                        [min, min + size / 2.0].into(),
                                        0.0,
                                        egui::Color32::DARK_GRAY,
                                        egui::Stroke::none(),
                                    );
                                    painter.rect(
                                        [
                                            min + (size.x / 2.0, 0.0).into(),
                                            min + (size.x, size.y / 2.0).into(),
                                        ]
                                        .into(),
                                        0.0,
                                        egui::Color32::LIGHT_GRAY,
                                        egui::Stroke::none(),
                                    );
                                    painter.rect(
                                        [
                                            min + (0.0, size.y / 2.0).into(),
                                            min + (size.x / 2.0, size.y).into(),
                                        ]
                                        .into(),
                                        0.0,
                                        egui::Color32::LIGHT_GRAY,
                                        egui::Stroke::none(),
                                    );
                                    painter.rect(
                                        [
                                            min + (size.x / 2.0, size.y / 2.0).into(),
                                            min + (size.x, size.y).into(),
                                        ]
                                        .into(),
                                        0.0,
                                        egui::Color32::DARK_GRAY,
                                        egui::Stroke::none(),
                                    );
                                } else {
                                    let [r, g, b] = pal[i as usize].to_rgb888();
                                    painter.rect(
                                        [min, min + size].into(),
                                        0.0,
                                        egui::Color32::from_rgb(r, g, b),
                                        egui::Stroke::none(),
                                    );
                                }
                                min.x += size.x;
                            }
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
