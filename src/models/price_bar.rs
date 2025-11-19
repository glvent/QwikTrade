#[derive(Clone, Debug)]
pub struct PriceBar {
    // Index acts as an x-position for each bar...
    pub index: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}

impl PriceBar {
    pub fn new(index: u64, open: f32, high: f32, low: f32, close: f32, volume: f32) -> Self {
        Self { index, open, high, low, close, volume }
    }
}