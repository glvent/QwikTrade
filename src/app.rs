use eframe::egui::Context;

use crate::ui::components::debug::draw_debug;
use crate::ui::{self, layout};
use crate::state::MarketState;
use crate::ui::theme;

pub struct App {
    pub symbol: String,
    pub timeframe: String,
    pub status_message: String,
    pub tick_counter: u64,
    pub market: MarketState,
    ui_init: bool,
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
            timeframe: "1H".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
            market,
            ui_init: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.ui_init {
            theme::default_theme(ctx);
            self.ui_init = true
        }
        
        self.tick_counter += 1;

        layout::draw_top(self, ctx);
        layout::draw_bottom(self, ctx);
        layout::draw_left(self, ctx);
        layout::draw_right(self, ctx);
        layout::draw_central(self, ctx);

        draw_debug(self, ctx);

        ctx.request_repaint();
    }
}