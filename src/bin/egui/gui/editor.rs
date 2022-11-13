use crate::{message, widgets::palette::PalPreview};
use eframe::{
    egui::{self, containers::Frame, text::LayoutJob, ComboBox, ScrollArea, TextFormat, Ui},
    epaint::Stroke,
};
use egui_extras::image::RetainedImage;
use nuclear::{
    img::{ncgr::NCGRTiles, NCGR, NCLR},
    proj::NuclearProject,
};

pub enum Editor {
    Palette {
        name: String,
        transparency: bool,
        contents: NCLR,
    },
    Tileset {
        name: String,
        contents: NCGR,
        palette: Option<String>,
        image: Option<RetainedImage>,
        view: ViewOptions,
    },
    Tilemap {
        name: String,
        tileset: Option<String>,
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

pub struct ViewOptions {
    pub width: u16,
    pub palette: i16,
    pub start_at: u32,
    pub length: u32,
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            width: 256,
            palette: -1,
            start_at: 0,
            length: 1,
        }
    }
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
    pub fn tileset(name: String, contents: NCGR, palette: Option<String>) -> Self {
        Self::Tileset {
            name,
            contents,
            image: if let Some(_) = palette {
                todo!();
            } else {
                None
            },
            palette,
            view: Default::default(),
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
    pub fn draw(&mut self, proj: &NuclearProject, ui: &mut Ui) -> EditorResponse {
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
            Self::Tileset {
                contents,
                palette,
                view,
                ..
            } => {
                ui.heading("Tileset editor");
                ui.label("(Only meant for previewing)\n");
                response = Self::draw_tileset(ui, proj, contents, palette, view);
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
                ui.set_width(350.0);
                ui.set_height(350.0);
                ui.vertical(|ui| {
                    if contents.is_8_bit {
                        ui.add(PalPreview {
                            color_amt: contents.color_amt,
                            palette: &contents.palettes[&0],
                            is_8_bit: contents.is_8_bit,
                            transparency: *transparency,
                        });
                    } else {
                        for (num, pal) in &contents.palettes {
                            ui.horizontal(|ui| {
                                ui.label(format!("Palette {}", num)).rect;
                                ui.add(PalPreview {
                                    color_amt: contents.color_amt,
                                    palette: pal,
                                    is_8_bit: contents.is_8_bit,
                                    transparency: *transparency,
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

    fn draw_tileset(
        ui: &mut Ui,
        project: &NuclearProject,
        contents: &NCGR,
        palette: &mut Option<String>,
        view: &mut ViewOptions,
    ) -> EditorResponse {
        ui.label("Palette associated with this tileset:");
        ComboBox::from_label("")
            .selected_text(palette.as_deref().unwrap_or("None"))
            .show_ui(ui, |ui| {
                ui.selectable_value(palette, None, "None");
                for (name, _) in &project.palette_sets {
                    ui.selectable_value(palette, Some(name.clone()), name);
                }
            });
        ui.label("");
        if contents.ncbr_ff {
            if let NCGRTiles::Lineal(_) = contents.tiles {
                let mut text = LayoutJob::default();
                let color = ui.style().visuals.widgets.noninteractive.text_color();
                text.append(
                    "WARNING:",
                    0.0,
                    TextFormat {
                        underline: Stroke { color, width: 1.0 },
                        color,
                        ..Default::default()
                    },
                );
                text.append(
                    " NCBR + lineal mode detected. Tiles may look garbled",
                    0.0,
                    TextFormat {
                        color,
                        ..Default::default()
                    },
                );
                ui.label(text);
            }
        }
        if let None = palette {
            ui.set_enabled(false);
        }
        ui.horizontal(|ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_size(egui::vec2(100.00, 100.0));
                ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.label("bloop");
                })
            });
            ui.vertical(|ui| {
                ComboBox::from_label("")
                    .selected_text(view.width.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut view.width, 8, "8");
                        ui.selectable_value(&mut view.width, 16, "16");
                        ui.selectable_value(&mut view.width, 32, "32");
                        ui.selectable_value(&mut view.width, 64, "64");
                        ui.selectable_value(&mut view.width, 256, "256");
                    });
            });
        });
        EditorResponse::None
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
