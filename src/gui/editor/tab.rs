use super::EditorType;
use eframe::egui::{Color32, Frame, Response, Style, Ui, Widget};

pub enum TabResponse {
    None,
    Close,
    Select,
}

pub struct Tab<'a> {
    pub name: &'a str,
    pub editor_type: EditorType,
    //TODO: editor inside the tab
}

impl Widget for Tab<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!();
    }
}

pub fn render_tab(ui: &mut Ui, tab: &(String, EditorType), selected: bool) -> TabResponse {
    let mut frame = Frame::group(&Style::default());
    if selected {
        frame = frame.fill(if ui.visuals().dark_mode {
            Color32::from_white_alpha(15)
        } else {
            Color32::LIGHT_GRAY
        });
    }
    let response = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("{} ({})", tab.0, tab.1));
            ui.button("X")
        })
    });
    if response.inner.inner.clicked() {
        println!("close tab");
        TabResponse::Close
    } else if response.response.clicked() {
        println!("select tab");
        TabResponse::Select
    } else {
        TabResponse::None
    }
}
