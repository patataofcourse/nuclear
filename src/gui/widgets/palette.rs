use crate::img::ColorBGR555;
use eframe::egui::{self, Color32, Response, Sense, TextStyle, Ui, Widget};

pub struct PalPreview<'a> {
    pub color_amt: u32,
    pub palette: &'a Vec<ColorBGR555>,
    pub is_8_bit: bool,
}

impl Widget for PalPreview<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // 1. Deciding widget size:
        //   Width: text_height * num_colors_row + separator_size * (num_colors_row - 1)
        //   Height: text_height * num_columns + separator_size * (num_columns - 1)
        let color_size = ui.fonts().pixels_per_point() * TextStyle::Body.resolve(ui.style()).size;
        const SEPARATOR_SIZE: f32 = 2.0;

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
            color_size * num_columns as f32 + SEPARATOR_SIZE * (num_columns as f32 - 1.0),
            color_size * num_rows as f32 + SEPARATOR_SIZE * (num_rows as f32 - 1.0),
        );

        // 2. Allocating space:
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // 3. Interact: Time to check for clicks!
        let clicked_color: Option<usize> = None;
        if response.clicked() {
            let pos = response.interact_pointer_pos().unwrap() - rect.min;

            // if it hasn't clicked on a separator
            if !(pos.x % (color_size + SEPARATOR_SIZE) >= color_size
                || pos.y % (color_size + SEPARATOR_SIZE) >= color_size)
            {
                let row = pos.y / 17.0;
                let column = pos.x / 17.0;
                dbg!(row, column, row as usize * 16 + column as usize);
            }
        }

        // Attach some meta-data to the response which can be used by screen readers:
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Other, true, format!("Palette previewer"))
            //TODO
        });

        // 4. Paint!
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect(rect, 0.0, Color32::BLACK, (0.0, Color32::BLACK));
            for i in 0..num_rows {
                //todo!();
            }
        }
        response
    }
}
