use eframe::egui::{Frame, CentralPanel, Context, SidePanel, TopBottomPanel, Color32, Rangef};

use crate::app::App;
use crate::ui::components::chart::draw_chart;
use crate::ui::components::toolbars::toolbar::Toolbar;

fn default_frame() -> Frame {
    Frame::default().fill(Color32::from_rgb(40, 40, 40)).inner_margin(5.0)
}

pub fn draw_central(app: &mut App, ctx: &Context) {
    CentralPanel::default()
        .frame(Frame::default().inner_margin(20.0).fill(Color32::from_rgb(40, 40, 40)))
        .show(ctx, |ui| {
            draw_chart(app, ui);

            ctx.request_repaint();
        });
}

pub fn draw_top(app: &mut App, ctx: &Context) {
    TopBottomPanel::top("top_bar")
        .frame(default_frame())
        .exact_height(60.0)
        .show(ctx, |ui| {
            Toolbar::TopToolbar.draw(app, ui);
        });
}

pub fn draw_left(app: &mut App, ctx: &Context) {
    SidePanel::left("left_panel")
        .resizable(false)
        .default_width(40.0)
        .frame(default_frame())
        .show(ctx, |ui| {
            Toolbar::LeftToolbar.draw(app, ui);
        });
}

pub fn draw_right(app: &mut App, ctx: &Context) {
    SidePanel::right("right_panel")
        .resizable(true)
        .default_width(200.0)
        .frame(default_frame())
        .show(ctx, |ui| {
            Toolbar::RightToolbar.draw(app, ui);
        });
}

pub fn draw_bottom(app: &mut App, ctx: &Context) {
    TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .exact_height(40.0)
        .frame(default_frame())
        .show(ctx, |ui| {
            Toolbar::BottomToolbar.draw(app, ui);
        });   
}
