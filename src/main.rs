mod app;
mod ui;

use eframe::egui;
use app::TradeApp;

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_title("TradeEZ");

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "TradeEZ", 
        native_options,
        Box::new(|_cc| Ok(Box::new(TradeApp::default()))),
    )
}
