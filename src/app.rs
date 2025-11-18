use eframe::egui;

use crate::ui::layout;

pub struct App {
    pub symbol: String,
    pub timeframe: String,
    pub status_message: String,
    pub tick_counter: u64,
}

impl Default for App {
    fn default() -> Self {
        // Placeholder data...
        App {
            symbol: "BTC-USDT".to_owned(),
            timeframe: "1m".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_counter += 1;

        layout::draw_top_panel(self, ctx);
        layout::draw_left_panel(self, ctx);
        layout::draw_central_panel(ctx);

        ctx.request_repaint();
    }
}