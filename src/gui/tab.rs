use eframe::egui::{
    self, style::Margin, Color32, FontSelection, Id, Rect, Response, Sense, Ui, Vec2, Widget,
    WidgetText,
};

#[derive(Clone, PartialEq, Debug)]
pub enum TabBarResponse {
    None,
    Select(usize),
    Close(usize),
}

pub struct Tab<'a> {
    pub name: &'a str,
    pub editor_type: &'static str,
    pub selected: bool,
}

impl Tab<'_> {
    fn selected_color(ui: &Ui) -> Color32 {
        if ui.visuals().dark_mode {
            Color32::from_white_alpha(15)
        } else {
            Color32::LIGHT_GRAY
        }
    }
}

impl Widget for Tab<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // 1. Deciding widget size:
        let margin = Margin::same(6.0);
        let text = WidgetText::from(format!("{} ({})", self.name, self.editor_type)).into_galley(
            ui,
            Some(false),
            f32::INFINITY,
            FontSelection::Default,
        );
        let button_size =
            ui.style().spacing.interact_size / Vec2::new(3.0, 1.5) + Vec2::new(0.0, 2.0);
        let button_offset = margin.left_top()
            + Vec2::new(text.size().x, 0.0)
            + Vec2::new(ui.style().spacing.item_spacing.x, 0.0);
        let desired_size = margin.left_top()
            + text.size()
            + Vec2::new(ui.style().spacing.item_spacing.x, 0.0)
            + Vec2::new(button_size.x, 0.0)
            + margin.right_bottom();

        // 2. Allocating space:
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        let button_rect = Rect {
            min: rect.min + button_offset,
            max: rect.min + button_offset + button_size,
        };
        let button_response = ui.interact(
            button_rect,
            Id::new(format!("tab_button_{}_{}", self.name, self.editor_type)),
            Sense::click(),
        );

        // 3. Interact: Time to check for clicks!
        if !button_response.clicked_elsewhere() && button_response.ctx.input().pointer.any_click()
        // because button_response.clicked() doesn't seem to work
        {
            response.mark_changed();
        }
        // Attach some meta-data to the response which can be used by screen readers:
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Button, true, format!("Tab"))
            //TODO
        });

        // 4. Paint!
        if ui.is_rect_visible(rect) {
            // background
            let mut visuals = ui.style().noninteractive().clone();
            let color = if self.selected {
                Self::selected_color(ui)
            } else {
                visuals.bg_fill
            };
            ui.painter()
                .rect(rect, visuals.rounding, color, visuals.fg_stroke);

            // text
            if ui.visuals().dark_mode && self.selected {
                visuals.fg_stroke.color = Color32::LIGHT_GRAY;
            }

            text.paint_with_visuals(ui.painter(), rect.left_top() + margin.left_top(), &visuals);

            // button
            let visuals = ui.style().interact(&button_response);
            ui.painter().rect(
                button_rect,
                visuals.rounding,
                visuals.bg_fill,
                visuals.bg_stroke,
            );

            // X
            let x_text =
                WidgetText::from("x").into_galley(ui, None, f32::INFINITY, FontSelection::Default);
            x_text.paint_with_visuals(
                ui.painter(),
                button_rect.min + Vec2::new(3.0, 0.25),
                visuals,
            )
        }
        response
    }
}
