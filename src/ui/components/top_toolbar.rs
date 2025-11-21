use eframe::egui::Ui;

use crate::app::App;

pub fn draw_top_toolbar(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        draw_title(app, ui);
        draw_symbol(app, ui);
        draw_timeframe(app, ui);
    });
}

fn draw_title(_app: &mut App, ui: &mut Ui) {
    ui.heading("QwikTrade");
}

fn draw_symbol(app: &mut App, ui: &mut Ui) {
    ui.label("Symbol:");
    ui.text_edit_singleline(&mut app.symbol);
}

fn draw_timeframe(app: &mut App, ui: &mut Ui) {
    ui.label("Timeframe:");
    ui.text_edit_singleline(&mut app.timeframe);
}