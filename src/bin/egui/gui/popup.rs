use eframe::{
    egui::{Context, Key, Layout, TextEdit, Window},
    emath::Align,
};

pub enum PopupResponse {
    None,
    Ok,
    Cancel,
}

pub enum PopupState {
    NameSelector {
        title: String,
        prompt: String,
        result: String,
    },
}

impl PopupState {
    pub fn spawn(&mut self, ctx: &Context) -> PopupResponse {
        match self {
            Self::NameSelector {
                title,
                prompt,
                result,
            } => name_selector(ctx, title, prompt, result),
        }
    }
}

pub fn name_selector(
    ctx: &Context,
    title: &str,
    prompt: &str,
    result: &mut String,
) -> PopupResponse {
    Window::new(title)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(prompt);
            let response = ui.add(TextEdit::singleline(result).desired_width(ui.available_width()));

            ui.label("");

            ui.with_layout(Layout::right_to_left(Align::default()), |ui| {
                if ui.button("Cancel").clicked() {
                    PopupResponse::Cancel
                } else if ui.button("Ok").clicked()
                    || (response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)))
                {
                    PopupResponse::Ok
                } else {
                    PopupResponse::None
                }
            })
            .inner
        })
        .map(|c| c.inner.unwrap_or(PopupResponse::None))
        .unwrap_or(PopupResponse::None)
}
