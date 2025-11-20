mod app;
mod config;
mod models;
mod ui;
mod state;

use app::App;
use config::load_config;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let conf = load_config();

    let viewport = egui::ViewportBuilder::default()
        .with_resizable(conf.window.resizable)
        .with_inner_size([conf.window.width as f32, conf.window.height as f32])
        .with_title(conf.window.title.clone());

    let native_opts = eframe::NativeOptions {
        viewport,
        vsync: conf.window.vsync,
        ..Default::default()
    };

    eframe::run_native(
        "QwikTrade",
        native_opts,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
