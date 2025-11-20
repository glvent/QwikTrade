use eframe::egui;

use crate::app::App;
use crate::ui::components::chart::draw_chart;
use crate::ui::components::top_toolbar::draw_top_toolbar;
use crate::ui::components::left_toolbar::draw_left_toolbar;
// use crate::ui::components::right_toolbar::draw_right_toolbar;

pub fn draw_central_panel(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_chart(app, ui);

        ctx.request_repaint();
    });
}

pub fn draw_top_panel(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        draw_top_toolbar(app, ui);
    });
}

pub fn draw_left_panel(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(225.0)
        .show(ctx, |ui| {
            draw_left_toolbar(ui);
        });
}

pub fn _draw_right_panel(_app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(225.0)
        .show(ctx, |ui| {
            // TODO: Implement right panel...
        });
}
