use eframe::egui::{
    self, popup, Color32, Id, Painter, Response, RichText, Sense, TextStyle, Ui, Widget,
};
use nuclear::img::ColorBGR555;

pub struct PalPreview<'a> {
    pub color_amt: u32,
    pub palette: &'a Vec<ColorBGR555>,
    pub is_8_bit: bool,
    pub transparency: bool,
}

fn transparency(painter: &Painter, pos: egui::Pos2, size: egui::Vec2) {
    painter.rect(
        [pos, pos + size / 2.0].into(),
        0.0,
        egui::Color32::DARK_GRAY,
        egui::Stroke::NONE,
    );
    painter.rect(
        [
            pos + (size.x / 2.0, 0.0).into(),
            pos + (size.x, size.y / 2.0).into(),
        ]
        .into(),
        0.0,
        egui::Color32::LIGHT_GRAY,
        egui::Stroke::NONE,
    );
    painter.rect(
        [
            pos + (0.0, size.y / 2.0).into(),
            pos + (size.x / 2.0, size.y).into(),
        ]
        .into(),
        0.0,
        egui::Color32::LIGHT_GRAY,
        egui::Stroke::NONE,
    );
    painter.rect(
        [
            pos + (size.x / 2.0, size.y / 2.0).into(),
            pos + (size.x, size.y).into(),
        ]
        .into(),
        0.0,
        egui::Color32::DARK_GRAY,
        egui::Stroke::NONE,
    );
}

impl Widget for PalPreview<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // 1. Deciding widget size:
        //   Width: text_height * num_colors_row + separator_size * (num_colors_row - 1)
        //   Height: text_height * num_columns + separator_size * (num_columns - 1)
        let color_size =
            ui.fonts(|fonts| fonts.pixels_per_point()) * TextStyle::Body.resolve(ui.style()).size;
        const SEPARATOR_SIZE: f32 = 1.0;

        let num_rows = if self.is_8_bit {
            self.color_amt / 16
        } else {
            1
        } as usize;

        let num_columns = if self.is_8_bit && self.color_amt > 16 {
            16
        } else {
            self.color_amt
        } as usize;

        let desired_size = egui::vec2(
            color_size * num_columns as f32 + SEPARATOR_SIZE * (num_columns as f32 + 1.0),
            color_size * num_rows as f32 + SEPARATOR_SIZE * (num_rows as f32 + 1.0),
        );

        // 2. Allocating space:
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // 3. Interact: Time to check for clicks!
        let mut clicked_color: Option<usize> = None;
        if response.clicked() {
            let pos = response.interact_pointer_pos().unwrap() - rect.min;

            // if it hasn't clicked on a separator
            if !(pos.x < SEPARATOR_SIZE
                || pos.y < SEPARATOR_SIZE
                || (pos.x - SEPARATOR_SIZE) % (color_size + SEPARATOR_SIZE) >= color_size
                || pos.y - SEPARATOR_SIZE % (color_size + SEPARATOR_SIZE) >= color_size)
            {
                let row = pos.y / 17.0;
                let column = pos.x / 17.0;
                clicked_color = Some(row as usize * 16 + column as usize);
            }
        } else if response.hovered() {
            let pos = response.hover_pos().unwrap() - rect.min;

            // if it hasn't hovered on a separator
            if !(pos.x < SEPARATOR_SIZE
                || pos.y < SEPARATOR_SIZE
                || (pos.x - SEPARATOR_SIZE) % (color_size + SEPARATOR_SIZE) >= color_size
                || pos.y - SEPARATOR_SIZE % (color_size + SEPARATOR_SIZE) >= color_size)
            {
                let row = pos.y / 17.0;
                let column = pos.x / 17.0;
                let hovered_color = row as usize * 16 + column as usize;
                let hovered_color_data = self.palette[hovered_color].to_rgb8();
                let hovered_color_hex = hovered_color_data[0] as u32 * 0x10000
                    + hovered_color_data[1] as u32 * 0x100
                    + hovered_color_data[2] as u32;

                popup::show_tooltip(ui.ctx(), Id::new("palette_tooltip"), |ui| {
                    ui.label(RichText::new(format!("#{hovered_color_hex:06x}")))
                });
            }
        }

        // Attach some meta-data to the response which can be used by screen readers:
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Other,
                true,
                "Palette previewer".to_string(),
            )
        });

        // 4. Paint!
        let visuals = ui.visuals().noninteractive();

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect(rect, 0.0, visuals.bg_stroke.color, (0.0, Color32::BLACK));
            for i in 0..num_rows {
                for j in 0..if self.is_8_bit && i == num_rows - 1 {
                    self.color_amt as usize - 16 * num_rows
                } else {
                    num_columns
                } {
                    let origin_pos = rect.min
                        + egui::vec2(
                            SEPARATOR_SIZE + j as f32 * (color_size + SEPARATOR_SIZE),
                            SEPARATOR_SIZE + i as f32 * (color_size + SEPARATOR_SIZE),
                        );
                    if i == 0 && j == 0 && self.transparency {
                        transparency(painter, origin_pos, egui::vec2(color_size, color_size))
                    } else {
                        let [r, g, b] = self.palette[i * 16 + j].to_rgb8();
                        painter.rect(
                            [origin_pos, origin_pos + egui::vec2(color_size, color_size)].into(),
                            0.0,
                            Color32::from_rgb(r, g, b),
                            (0.0, Color32::BLACK),
                        );
                    }
                }
            }
        }
        response
    }
}
