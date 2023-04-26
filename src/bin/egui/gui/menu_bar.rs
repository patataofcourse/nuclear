use std::path::PathBuf;

use crate::{gui::NuclearApp, message};
use eframe::egui::{menu, widgets, Align, Button, Context, Layout, TopBottomPanel};
use nuclear::extend::FormatType;

pub enum MenuBarResponse {
    NewProj,
    OpenProj,
    Metadata,
    None,
    ImportFile(FormatType),
}

impl MenuBarResponse {
    pub fn set_if_none(&mut self, other: Self) {
        if let MenuBarResponse::None = self {
            *self = other;
        }
    }
}

#[must_use]
pub fn menu_bar(app: &mut NuclearApp, ctx: &Context) -> MenuBarResponse {
    let mut response = MenuBarResponse::None;
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    response.set_if_none(MenuBarResponse::NewProj)
                }
                if ui.button("Open").clicked() {
                    response.set_if_none(MenuBarResponse::OpenProj)
                }

                ui.set_enabled(app.project.is_some());

                if ui.button("Save as").clicked() {
                    message::warning("Not implemented!", "Can't 'save as' yet")
                }
                /*
                ui.separator();
                ui.menu_button("Open recent", |ui| {
                    ui.button("1. -");
                    ui.button("2. -");
                });
                ui.button("Import portable project");
                ui.button("Export portable project");
                ui.separator();
                */
                ui.menu_button("Import", |ui| {
                    if ui.button("Nintendo files").clicked() {
                        response.set_if_none(MenuBarResponse::ImportFile(FormatType::Nintendo))
                    }
                    //ui.button("BNCAD");
                });
                /*
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
                    .add_enabled(app.project.is_none(), Button::new("Project metadata"))
                    .clicked()
                {
                    response.set_if_none(MenuBarResponse::Metadata)
                }
            });

            #[cfg(debug_assertions)]
            ui.menu_button("Debug", |ui| {
                if ui.button("rockers project").clicked() {
                    let path = PathBuf::from("test_files/rockers");
                    if app.close_project() {
                        let proj = nuclear::proj::NuclearProject::load_from_file(path).unwrap();
                        app.project = Some(proj)
                    }
                }
            });

            //TODO: where do i hide this easter egg?
            if ui.button("button!!!").clicked() {
                message::info("button", ":)")
            }

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.label("Toggle dark mode");
            });
        });
    });
    response
}
