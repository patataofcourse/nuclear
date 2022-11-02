use eframe::egui::{InnerResponse, Response, Ui};

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
                Self::Palette {} => ui.label("Not implemented"),
                Self::Tileset {} => ui.label("Not implemented"),
                Self::Tilemap {} => ui.label("Not implemented"),
                Self::Frames {} => ui.label("Not implemented"),
                Self::Animation {} => ui.label("Not implemented"),
            }
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
