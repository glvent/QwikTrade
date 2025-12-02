mod app;
mod config;
mod models;
mod ui;
mod state;

use app::App;
use config::load_config;
use eframe::egui::{IconData, ViewportBuilder};

fn load_icon() -> IconData {
    let bytes = std::fs::read("assets/icon.png").expect("Icon not found.");
    let img = image::load_from_memory(&bytes).expect("Invalid icon image.");
    
    let resized_img = img.resize_exact(512, 512, image::imageops::FilterType::Lanczos3);
    let rgba_img = resized_img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    println!("Width: {}, Height: {}", width, height);

    let rgba = rgba_img.into_raw();

    IconData { 
        rgba,
        width,
        height,
     }
}

fn main() -> eframe::Result<()> {
    let conf = load_config();

    let icon = load_icon();

    let viewport = ViewportBuilder::default()
        .with_resizable(conf.window.resizable)
        .with_inner_size([conf.window.width as f32, conf.window.height as f32])
        .with_title(conf.window.title.clone())
        .with_icon(icon);

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
