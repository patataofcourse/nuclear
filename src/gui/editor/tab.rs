use super::EditorType;
use eframe::egui::{self, style::Margin, Color32, Frame, Response, Style, Ui, Widget};

pub enum TabResponse {
    None,
    Close,
    Select,
}

pub struct Tab<'a> {
    pub name: &'a str,
    pub editor_type: EditorType,
    pub selected: bool,
    //TODO: editor inside the tab
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
        // Widget code can be broken up in four steps:
        //  1. Decide a size for the widget
        //  2. Allocate space for it
        //  3. Handle interactions with the widget (if any)
        //  4. Paint the widget
        // 1. Deciding widget size:
        // You can query the `ui` how much space is available,
        // but in this example we have a fixed size widget based on the height of a standard button:
        let margin = Margin::same(6.0);
        let desired_size =
            ui.spacing().interact_size * egui::vec2(2.0, 1.0) + margin.right_bottom();

        // 2. Allocating space:
        // This is where we get a region of the screen assigned.
        // We also tell the Ui to sense clicks in the allocated region.
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // 3. Interact: Time to check for clicks!
        if response.clicked() {
            response.mark_changed(); // report back that the value changed
        }

        // Attach some meta-data to the response which can be used by screen readers:
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Button, true, format!("Tab"))
            //TODO
        });

        // 4. Paint!
        // Make sure we need to paint:
        if ui.is_rect_visible(rect) {
            // Let's ask for a simple animation from egui.
            // egui keeps track of changes in the boolean associated with the id and
            // returns an animated value in the 0-1 range for how much "on" we are.
            let visuals = ui.style().noninteractive();
            // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
            let color = if self.selected {
                Self::selected_color(ui)
            } else {
                visuals.bg_fill
            };
            ui.painter()
                .rect(rect, visuals.rounding, color, visuals.fg_stroke);
        }

        // All done! Return the interaction response so the user can check what happened
        // (hovered, clicked, ...) and maybe show a tooltip:
        response
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
