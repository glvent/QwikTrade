use eframe::egui::{Area, Color32, Context, Id, Order, pos2};

use crate::app::App;

pub fn draw_debug(app: &mut App, ctx: &Context) {
    let color = Color32::from_rgba_unmultiplied(235, 235, 235, 50);

    let content_rect = ctx.available_rect();
    let debug_pos = pos2(content_rect.right() - 40.0, content_rect.top() - 100.0);
    
    Area::new(Id::new("debug_area"))
        .fixed_pos(debug_pos)
        .order(Order::Foreground)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.colored_label(color, format!("Status: {}", app.status_message));
                ui.colored_label(color, format!("Ticks: {}", app.tick_counter));
            });
    });
}