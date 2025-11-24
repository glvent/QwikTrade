use eframe::egui::{RichText, Ui};

use crate::app::App;

pub fn draw(app: &mut App, ui: &mut Ui) {
    ui.horizontal_centered(|ui| {
        ui.horizontal(|ui| {
            draw_title(app, ui);
            draw_symbol(app, ui);
            draw_timeframe(app, ui);
        });
    });
}

fn draw_title(_app: &mut App, ui: &mut Ui) {
    ui.heading(RichText::new("QwikTrade").heading().size(18.0));
}
fn draw_symbol(app: &mut App, ui: &mut Ui) {
    ui.text_edit_singleline(&mut app.symbol);
}

fn draw_timeframe(app: &mut App, ui: &mut Ui) {
    ui.text_edit_singleline(&mut app.timeframe);
}
