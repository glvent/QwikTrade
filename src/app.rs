use eframe::egui;

use crate::ui::layout;
use crate::state::MarketState;

pub struct App {
    pub symbol: String,
    pub timeframe: String,
    pub status_message: String,
    pub tick_counter: u64,
    pub market: MarketState,
}

impl Default for App {
    fn default() -> Self {
        // Placeholder data...
        // Eventually, symbol and timeframe might be abstracted...
        let mut market = MarketState::new(30_000.0);
        
        // Generate all bars immediately at startup
        market.generate_fake_data(1_000);
        
        App {
            symbol: "BTC-USDT".to_owned(),
            timeframe: "1m".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
            market,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_counter += 1;

        layout::draw_top_panel(self, ctx);
        layout::draw_left_panel(self, ctx);
        layout::draw_central_panel(self, ctx);
        //layout::draw_right_panel(self, ctx);
        
        ctx.request_repaint();
    }
}