use eframe::egui;

use crate::ui::layout;

pub struct TradeApp {
    pub symbol: String,
    pub timeframe: String,
    pub status_message: String,
    pub tick_counter: u64,
}

impl Default for TradeApp {
    fn default() -> Self {
        // Placeholder data...
        TradeApp {
            symbol: "BTC-USDT".to_owned(),
            timeframe: "1m".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
        }
    }
}

impl eframe::App for TradeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_counter += 1;

        layout::draw_top_panel(self, ctx);
        layout::draw_central_panel(ctx);

        ctx.request_repaint();
    }
}