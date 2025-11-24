use crate::models::PriceBar;
use rand::Rng;

pub struct MarketState {
    price_bars: Vec<PriceBar>,
    current_price: f32,
    next_index: u64,
}

impl MarketState {
    pub fn new(initial_price: f32) -> Self {
        Self {
            price_bars: Vec::new(),
            current_price: initial_price,
            next_index: 0,
        }
    }

    // Read-only view for UI...
    pub fn price_bars(&self) -> &[PriceBar] {
        &self.price_bars
    }

    // Creates placeholder OHLCV data...
    pub fn generate_fake_data(&mut self, max_days_generated: usize) {
        // Configurable parameters...
        const VOLATILITY: f32 = 0.005;
        const WICK_EXTRA: f32 = 0.002;
        const BASE_VOLUME: f32 = 1_000.0;
        const VOL_VOLUME: f32 = 500.0;

        let mut rng = rand::rng();

        for _ in 0..max_days_generated {
            let random_step = (rng.random::<f32>() - 0.5) * 2.0 * VOLATILITY;
            
            let open = self.current_price;
            let close = open * (1.0 + random_step);
            let wick_up = (rng.random::<f32>()) * WICK_EXTRA;
            let wick_down = (rng.random::<f32>()) * WICK_EXTRA;
            let high = open.max(close) * (1.0 + wick_up);
            let low = open.min(close) * (1.0 - wick_down);
            
            let vol_noise: f32 = (rng.random::<f32>() - 0.5) * 2.0 * VOL_VOLUME;
            let volume = (BASE_VOLUME + vol_noise).max(0.0);
            
            let bar = PriceBar::new(self.next_index, open, high, low, close, volume);
            self.price_bars.push(bar);
            
            self.current_price = close;
            self.next_index += 1;
        }
    }
}
