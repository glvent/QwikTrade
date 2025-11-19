use eframe::egui;

use crate::models::PriceBar;
use crate::ui::layout;

pub struct App {
    pub symbol: String,
    pub timeframe: String,
    pub status_message: String,
    pub tick_counter: u64,
    pub price_bars: Vec<PriceBar>,
    pub current_price: f32,
    pub next_bar_index: u64,
}

impl Default for App {
    fn default() -> Self {
        // Placeholder data...
        App {
            symbol: "BTC-USDT".to_owned(),
            timeframe: "1m".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
            price_bars: Vec::new(),
            current_price: 30_000.0,
            next_bar_index: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_counter += 1;

        self.generate_fake_data();

        layout::draw_top_panel(self, ctx);
        layout::draw_left_panel(self, ctx);
        layout::draw_central_panel(self, ctx);

        ctx.request_repaint();
    }
}

impl App {
    // Creates placeholder OHLCV data...
    fn generate_fake_data(&mut self) {
        if self.tick_counter % 5 != 0 {
            return;
        }
        // Configurable parameters...
        const MAX_BARS_DISPLAYED: usize = 200;
        const VOLATILITY: f32 = 0.004;
        const WICK_EXTRA: f32 = 0.002;
        const BASE_VOLUME: f32 = 1_000.0;
        const VOL_VOLUME: f32 = 500.0;

        use rand::Rng;

        let mut rng = rand::rng();

        let random_step = (rng.random::<f32>() - 0.5) * 2.0 * VOLATILITY;

        let open = self.current_price;
        let close = open * (1.0 + random_step);
        let wick_up = (rng.random::<f32>()) * WICK_EXTRA;
        let wick_down = (rng.random::<f32>()) * WICK_EXTRA;
        let high = open.max(close) * (1.0 + wick_up);
        let low = open.min(close) * (1.0 - wick_down);

        let vol_noise: f32 = (rng.random::<f32>() - 0.5) * 2.0 * VOL_VOLUME;
        let volume = (BASE_VOLUME + vol_noise).max(0.0);

        let index = self.next_bar_index;
        self.next_bar_index += 1;

        let bar = PriceBar::new(index, open, high, low, close, volume);
        self.price_bars.push(bar);

        self.current_price = close;

        if self.price_bars.len() > MAX_BARS_DISPLAYED {
            let excess = self.price_bars.len() - MAX_BARS_DISPLAYED;
            self.price_bars.drain(0..excess);
        }
    }
}
