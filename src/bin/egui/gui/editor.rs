use crate::{addon::NuclearResult, message, widgets::palette::PalPreview};
use eframe::{
    egui::{
        self, containers::Frame, text::LayoutJob, ComboBox, ScrollArea, Slider, TextFormat, Ui,
    },
    epaint::{ColorImage, Stroke},
};
use egui_extras::image::RetainedImage;
use nuclear::{
    error::Error,
    img::{ncgr::NCGRTiles, ColorBGR555, NCGR, NCLR},
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

#[derive(Clone, Debug)]
pub struct ViewOptions {
    pub width: usize,
    pub palette: isize,
    pub sectioned: bool,
    pub start_at: usize,
    pub length: usize,
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            width: 256,
            palette: -1,
            sectioned: false,
            start_at: 0,
            length: 4,
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
    SaveTset,
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
                image,
                ..
            } => {
                ui.heading("Tileset editor");
                response = Self::draw_tileset(ui, proj, contents, palette, view, image);
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
        image: &mut Option<RetainedImage>,
    ) -> EditorResponse {
        let mut response = EditorResponse::None;
        let mut update_img = false;

        ui.label("Palette associated with this tileset:");
        let before = palette.clone();
        ComboBox::from_label("")
            .selected_text(palette.as_deref().unwrap_or("None"))
            .show_ui(ui, |ui| {
                ui.selectable_value(palette, None, "None");
                for (name, _) in &project.palette_sets {
                    ui.selectable_value(palette, Some(name.clone()), name);
                }
            });
        if before != *palette {
            update_img = true;
        }
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
                if let Some(img) = image {
                    ui.set_min_height(img.height().min(512) as f32);
                    ScrollArea::new([false, true]).show(ui, |ui| {
                        ui.image(
                            img.texture_id(ui.ctx()),
                            [img.width() as f32, img.height() as f32],
                        );
                    });
                } else {
                    ui.set_height(100.0);
                    ui.label("Could not render image\nTry selecting a palette");
                }
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Palette");
                    let before = view.palette;
                    ComboBox::new("pal_combobox", "")
                        .selected_text(if view.palette >= 0 {
                            format!("Palette {}", view.palette)
                        } else {
                            "None".to_string()
                        })
                        .show_ui(ui, |ui| {
                            if let Some(c) = palette {
                                for (k, _) in project.get_nclr(c).manage().unwrap().palettes {
                                    ui.selectable_value(
                                        &mut view.palette,
                                        k as isize,
                                        format!("Palette {}", k),
                                    );
                                }
                            } else {
                            }
                        });
                    if before != view.palette {
                        update_img = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Display width");
                    let before = view.width;
                    ComboBox::new("width_combobox", "")
                        .selected_text(format!("{} px", view.width))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut view.width, 8, "8 px");
                            ui.selectable_value(&mut view.width, 16, "16 px");
                            ui.selectable_value(&mut view.width, 32, "32 px");
                            ui.selectable_value(&mut view.width, 64, "64 px");
                            ui.selectable_value(&mut view.width, 256, "256 px");
                        });
                    if before != view.width {
                        update_img = true;
                    }
                });
                if ui.checkbox(&mut view.sectioned, "View section").changed() {
                    update_img = true;
                }
                ui.set_enabled(view.sectioned);
                ui.horizontal(|ui| {
                    ui.label("Start at:");
                    if ui
                        .add(Slider::new(
                            &mut view.start_at,
                            0..=(contents.tiles.len(contents.is_8_bit) - 1),
                        ))
                        .changed()
                    {
                        update_img = true;
                    }
                    ui.label("tiles");
                });
                ui.horizontal(|ui| {
                    ui.label("Length:");
                    if ui
                        .add(Slider::new(
                            &mut view.length,
                            1..=(contents.tiles.len(contents.is_8_bit) - view.start_at),
                        ))
                        .changed()
                    {
                        update_img = true;
                    }
                    ui.label("tiles");
                })
            });
        });
        if ui.button("Save").clicked() {
            response = EditorResponse::SaveTset;
        }

        if update_img {
            Self::update_tileset_img(contents, project, palette, image, view);
        }

        response
    }

    fn update_tileset_img(
        ncgr: &NCGR,
        project: &NuclearProject,
        palette: &Option<String>,
        image: &mut Option<RetainedImage>,
        view: &ViewOptions,
    ) {
        if let Some(c) = palette {
            let nclr = project.get_nclr(c).manage().unwrap();
            if view.palette >= 0 && nclr.palettes.contains_key(&(view.palette as u16)) {
                let img = ncgr.tiles.render(
                    ncgr.is_8_bit,
                    if view.sectioned {
                        let end = (view.start_at + view.length).min(ncgr.tiles.len(ncgr.is_8_bit));
                        Some(view.start_at..end)
                    } else {
                        None
                    },
                    view.width as usize / 8,
                );

                let pal = nclr.palettes.get(&(view.palette as u16)).unwrap();
                let mut rgba = vec![];

                for px in &img {
                    rgba.extend(
                        pal.get(*px as usize)
                            .ok_or(Error::MalformedData {
                                file: "{current tileset}".to_string(),
                            })
                            .manage()
                            .to_rgb888(),
                    );
                    rgba.push(255);
                }

                while rgba.len() % (view.width * 4) != 0 {
                    rgba.push(0);
                }

                let height = if img.len() % view.width == 0 {
                    img.len() / view.width
                } else {
                    img.len() / view.width + 1
                };

                *image = Some(RetainedImage::from_color_image(
                    "texture",
                    ColorImage::from_rgba_unmultiplied([view.width, height], &rgba),
                ));
            } else {
                *image = None
            }
        } else {
            *image = None
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