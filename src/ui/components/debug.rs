use eframe::egui::{Area, Color32, Ui, Id, Order, pos2};

use crate::app::App;

pub fn draw_debug(app: &mut App, ui: &mut Ui) {
    let color = Color32::from_rgba_unmultiplied(238, 238, 238, 50);

    let chart_rect = ui.available_rect_before_wrap();
    let debug_pos = pos2(chart_rect.right(), chart_rect.bottom() - 40.0);
    
    Area::new(Id::new("debug_area"))
        .fixed_pos(debug_pos)
        .order(Order::Foreground)
        .show(ui.ctx(), |ui| {
            ui.vertical(|ui| {
                ui.colored_label(color, format!("Status: {}", app.status_message));
                ui.colored_label(color, format!("Tick Counter: {}", app.tick_counter));
            });
    });
}