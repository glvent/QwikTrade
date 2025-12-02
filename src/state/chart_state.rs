pub struct ChartState {
    pub hover_price: Option<f32>,
    pub hover_index: Option<usize>,
    pub pan_offset: f32,
    pub pan_drag_start: f32,
    pub drag_start_x: Option<f32>,
    pub zoom: f32,
}

impl Default for ChartState {
    fn default() -> Self {
        Self {
            hover_price: None, 
            hover_index: None,
            pan_offset: 0.0,
            pan_drag_start: 0.0,
            drag_start_x: None,
            zoom: 1.0,
        }
    }
}

impl ChartState {
    pub fn new() -> Self {
        Self::default()
    }
}