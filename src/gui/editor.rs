use eframe::egui::{containers::Frame, InnerResponse, Response, Ui};

#[derive(Clone, Debug)]
pub enum Editor {
    Palette {},
    Tileset {},
    Tilemap {},
    Frames {},
    Animation {},
}

#[derive(Debug, Clone)]
pub struct PaletteEditor {
    pub name: String,
}

impl Editor {
    pub const fn editor_type(&self) -> &'static str {
        match self {
            Self::Palette { .. } => "Palette",
            Self::Tileset { .. } => "Tileset",
            Self::Tilemap { .. } => "Tilemap",
            Self::Frames { .. } => "Frames",
            Self::Animation { .. } => "Animation",
        }
    }

    pub fn draw(&self, ui: &mut Ui) -> InnerResponse<Response> {
        ui.vertical(|ui| {
            ui.heading(format!("{} editor", self.editor_type()));
            match &self {
                Self::Palette {} => {
                    ui.horizontal(|ui| {
                        Frame::group(ui.style()).show(ui, |ui| {
                            ui.set_width(200.0);
                            ui.set_height(200.0);
                            ui.vertical(|ui| {
                                ui.label("Palette 0");
                                ui.label("Palette 1");
                                //TODO
                            })
                        });
                        ui.vertical(|ui| {
                            ui.button("Import .pal file");
                            ui.button("Export .pal file");
                        })
                    });
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
            }
            ui.label("")
        })
    }
}

impl PaletteEditor {
    pub fn draw(&self, ui: &mut Ui) -> InnerResponse<Response> {
        ui.vertical(|ui| {
            ui.heading(format!("Palette {}", self.name));
            ui.button("hehehe")
        })
    }
}
