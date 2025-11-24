pub struct ChartState {
    pub hover_price: Option<f32>,
    pub hover_index: Option<usize>,
    pub pan_offset: f32,
    pub zoom: f32,
    pub start_index: usize,
}

impl Default for ChartState {
    fn default() -> Self {
        Self {
            hover_price: None, 
            hover_index: None,
            pan_offset: 0.0,
            zoom: 1.0,
            start_index: 0,
        }
    }
}

impl ChartState {
    pub fn new() -> Self {
        Self::default()
    }
}