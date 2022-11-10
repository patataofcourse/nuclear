use crate::gui::NuclearApp;
use eframe::egui::{menu, widgets, Align, Button, Context, Layout, TopBottomPanel};

pub enum MenuBarResponse {
    NewProj,
    OpenProj,
    Metadata,
    None,
}

impl MenuBarResponse {
    pub fn set_if_none(&mut self, other: Self) {
        if let MenuBarResponse::None = self {
            *self = other;
        }
    }
}

#[must_use]
pub fn menu_bar(app: &NuclearApp, ctx: &Context) -> MenuBarResponse {
    let mut response = MenuBarResponse::None;
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    response.set_if_none(MenuBarResponse::NewProj)
                };
                if ui.button("Open").clicked() {
                    response.set_if_none(MenuBarResponse::OpenProj)
                };
                ui.button("Save as");
                /*
                ui.separator();
                ui.menu_button("Open recent", |ui| {
                    ui.button("1. -");
                    ui.button("2. -");
                });
                ui.button("Import portable project");
                ui.button("Export portable project");
                ui.separator();
                ui.menu_button("Import", |ui| {
                    ui.button("Nintendo files");
                    ui.button("BNCAD");
                });
                ui.menu_button("Export", |ui| {
                    ui.button("Nintendo files");
                    ui.button("BNCAD");
                });
                */
            });
            ui.menu_button("Edit", |ui| {
                /*
                ui.button("Undo");
                ui.button("Redo");
                ui.separator();
                */
                if ui
                    .add_enabled(
                        if let None = app.project { false } else { true },
                        Button::new("Project metadata"),
                    )
                    .clicked()
                {
                    response.set_if_none(MenuBarResponse::Metadata)
                }
            });
            #[allow(unused_must_use)]
            {
                ui.button("button!!!");
            }
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.label("Toggle dark mode");
            });
        });
    });
    response
}
