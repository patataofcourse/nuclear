use eframe::egui::{Color32, Frame, InnerResponse, Style, Ui};
use std::fmt::Display;

pub mod nclr;

pub enum EditorType {
    Palette,
    Tileset,
    Tilemap,
    Frame,
    Animation,
}

impl Display for EditorType {
    //TODO: localization
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Palette => "Palette",
                Self::Tileset => "Tileset",
                Self::Tilemap => "Tilemap",
                Self::Frame => "Frames",
                Self::Animation => "Animations",
            }
        )
    }
}

pub fn render_tab(ui: &mut Ui, tab: &(String, EditorType), selected: bool) -> InnerResponse<()> {
    let mut frame = Frame::group(&Style::default());
    if selected {
        frame = frame.fill(if ui.visuals().dark_mode {
            Color32::from_white_alpha(15)
        } else {
            Color32::LIGHT_GRAY
        });
    }
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.visuals_mut();
            ui.label(format!("{} ({})", tab.0, tab.1));
            ui.button("X");
        });
    })
}
