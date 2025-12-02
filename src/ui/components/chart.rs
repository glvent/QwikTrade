use eframe::egui::{
    Align2, Color32, Painter, Pos2, Rect, Sense, Stroke, StrokeKind, TextStyle, Ui, Shape
};

use crate::app::App;
use crate::models::PriceBar;
use crate::state::{ChartState, MarketState};

pub fn draw_chart(app: &mut App, ui: &mut Ui) {
    let available_size = ui.available_size();
    let (rect, res) = ui.allocate_exact_size(available_size, Sense::hover());
    let painter = ui.painter_at(rect);

    let market: &mut MarketState = &mut app.market;
    let chart: &mut ChartState = &mut app.chart;

    let price_bars = market.price_bars();

    draw_chart_background(&painter, rect);

    if price_bars.is_empty() {
        draw_loading_message(&painter, rect, ui);
        return;
    }

    // Handle pinch zoom...
    if res.hovered() {
        let zoom_delta = ui.input(|i| i.zoom_delta());
        if zoom_delta != 1.0 {
            chart.zoom = (chart.zoom * zoom_delta).max(0.1).min(10.0);
        }
    }

    if res.dragged() {
        let drag_delta = res.drag_delta();
        
        const BASE_CANDLE_WIDTH: f32 = 6.0;
        const BASE_GAP: f32 = 1.0;

        let total_candle = (BASE_CANDLE_WIDTH + BASE_GAP) * chart.zoom;
        let total_chart_width = (price_bars.len() as f32) * total_candle;
        let vis_width = rect.width();

        let new_pan = chart.pan_offset + drag_delta.x;

        // Clamp pan to chart...
        let max_pan = 0.0;
        let min_pan = -(total_chart_width - vis_width).min(0.0);

        chart.pan_offset = new_pan.max(min_pan).min(max_pan);
    }

    if let Some(pointer_pos) = res.hover_pos() {
        handle_mouse_hover(chart, pointer_pos, rect, price_bars);
    }

    if let Some(price) = chart.hover_price {
        draw_hover_price(&painter, rect, price, price_bars, &chart);
    }

    draw_candles(rect, &painter, price_bars, &chart);
    draw_price_labels(&painter, rect, ui, price_bars);
    draw_current_price(&painter, rect, market.current_price(), price_bars);
}

fn draw_chart_background(painter: &Painter, rect: Rect) {
    painter.rect_filled(rect, 4.0, Color32::from_rgb(20, 20, 20));

    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(0.5, Color32::from_rgb(20, 20, 20)),
        StrokeKind::Outside,
    );
}

fn draw_loading_message(painter: &Painter, rect: Rect, ui: &Ui) {
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        "Loading data...",
        TextStyle::Heading.resolve(ui.style()),
        Color32::from_rgb(235, 235, 235),
    );
}

fn draw_candles(rect: Rect, painter: &Painter, price_bars: &[PriceBar], chart: &ChartState) {
    let (min_price, max_price) = get_price_bounds(price_bars);
    let price_span = (max_price - min_price).max(1.0);

    // Candle sizing constants...
    const BASE_CANDLE_WIDTH: f32 = 6.0;
    const BASE_GAP: f32 = 1.0;

    let candle_width = BASE_CANDLE_WIDTH * chart.zoom;
    let gap = BASE_GAP * chart.zoom;
    let total_candle = candle_width + gap;

    let vis_width = rect.width();
    let vis_candle_count = (vis_width / total_candle).ceil() as usize;

    let start_offset = (-chart.pan_offset / total_candle).floor() as usize;
    let start_index = start_offset.min(price_bars.len().saturating_sub(1));
    let end_index = (start_index + vis_candle_count).min(price_bars.len());

    /* TODO: implement as a method after abstracting logic further...
    // Handles which bars to show...
    let cap = (rect.width() / TOTAL_CANDLE).floor() as usize;
    let len = price_bars.len();
    let start = if len > cap { len - cap } else { 0 };
    let bars = &price_bars[start..];
    */

    for (i, bar) in price_bars[start_index..end_index].iter().enumerate() {
        let bar_index = start_index + i;
        let x = rect.left() + (bar_index as f32) * total_candle + chart.pan_offset;
        let x_center = x + candle_width * 0.5;

        let y_open = get_price_position_y(bar.open, min_price, price_span, rect);
        let y_close = get_price_position_y(bar.close, min_price, price_span, rect);
        let y_high = get_price_position_y(bar.high, min_price, price_span, rect);
        let y_low = get_price_position_y(bar.low, min_price, price_span, rect);

        let candle_color = get_price_color(bar.close >= bar.open);

        draw_candle(
            painter,
            x,
            x_center,
            y_open,
            y_close,
            y_high,
            y_low,
            candle_width,
            candle_color,
        );
    }

    /* Optional: show min/max labels.
    draw_price_labels(painter, rect, ui, min_price, max_price);
    */
}

fn draw_candle(
    painter: &Painter,
    x: f32,
    x_center: f32,
    y_open: f32,
    y_close: f32,
    y_high: f32,
    y_low: f32,
    candle_width: f32,
    color: Color32,
) {
    // Draw candle wick...
    painter.line_segment(
        [
            /* In the future when we introduce custom price bar coloring
            we will need 2 separate line segments for high and low... */
            Pos2::new(x_center, y_high),
            Pos2::new(x_center, y_low),
        ],
        Stroke::new(1.0, color),
    );

    // Draw candle body...
    let top = y_open.min(y_close);
    let bottom = y_open.max(y_close);
    let body_rect = Rect::from_two_pos(Pos2::new(x, top), Pos2::new(x + candle_width, bottom));

    painter.rect_filled(body_rect, 1.0, color);
    painter.rect_stroke(body_rect, 1.0, Stroke::new(0.2, color), StrokeKind::Outside);
}

fn draw_price_labels(painter: &Painter, rect: Rect, ui: &Ui, price_bars: &[PriceBar]) {
    let (min_price, max_price) = get_price_bounds(price_bars);

    painter.text(
        Pos2::new(rect.left() + 4.0, rect.top() + 4.0),
        Align2::LEFT_TOP,
        format!("{:.2}", max_price),
        TextStyle::Body.resolve(ui.style()),
        Color32::from_rgba_unmultiplied(200, 200, 200, 100),
    );
    painter.text(
        Pos2::new(rect.left() + 4.0, rect.bottom() - 4.0),
        Align2::LEFT_BOTTOM,
        format!("{:.2}", min_price),
        TextStyle::Body.resolve(ui.style()),
        Color32::from_rgba_unmultiplied(200, 200, 200, 100),
    );
} 

fn draw_hover_price(
    painter: &Painter,
    rect: Rect,
    current_price: f32,
    price_bars: &[PriceBar],
    chart: &ChartState,
) {
    if price_bars.is_empty() {
        return ();
    }

    const CANDLE_WIDTH: f32 = 6.0;
    const GAP: f32 = 1.0;
    const TOTAL_CANDLE: f32 = CANDLE_WIDTH + GAP;

    let hover_index = chart.hover_index;

    let x = rect.left() + (hover_index.unwrap() as f32) * TOTAL_CANDLE;
    let x_center = x + CANDLE_WIDTH * 0.5;

    let (min_price, max_price) = get_price_bounds(price_bars);
    let price_span = (max_price - min_price).max(1.0);
    let y = get_price_position_y(current_price, min_price, price_span, rect);

    let x_seg = [Pos2::new(x_center, rect.top()), Pos2::new(x_center, rect.bottom())];
    let y_seg = [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)];

    let stroke = Stroke {
        width: 1.0,
        color: Color32::from_rgba_unmultiplied(100, 100, 100, 200),
    };

    painter.add(Shape::dashed_line(
        &x_seg,
        stroke,
        4.0,
        4.0,
    ));

    painter.add(Shape::dashed_line(
        &y_seg,
        stroke,
        4.0,
        4.0,
    ));

    painter.text(
        Pos2::new(rect.right() - 60.0, y - 10.0),
        Align2::RIGHT_CENTER,
        format!("{:.2}", current_price),
        TextStyle::Small.resolve(&painter.ctx().style()),
        Color32::from_rgb(100, 100, 100),
    );
}

fn draw_current_price(
    painter: &Painter,
    rect: Rect,
    current_price: f32,
    price_bars: &[PriceBar],
) {
    if price_bars.is_empty() {
        return;
    }

    let (min_price, max_price) = get_price_bounds(price_bars);
    let price_span = (max_price - min_price).max(1.0);
    let y = get_price_position_y(current_price, min_price, price_span, rect);

    let line_segment = [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)];
    let stroke = Stroke {
        width: 1.0,
        color: get_price_color(current_price >= price_bars[0].close),
    };

    painter.add(Shape::dashed_line(
        &line_segment,
        stroke,
        4.0,
        4.0,
    ));

    painter.text(
        Pos2::new(rect.right() - 60.0, y - 10.0),
        Align2::RIGHT_CENTER,
        format!("{:.2}", current_price),
        TextStyle::Small.resolve(&painter.ctx().style()),
        get_price_color(current_price >= price_bars[0].close),
    );
}

fn get_price_bounds(price_bars: &[PriceBar]) -> (f32, f32) {
    let mut min_price = f32::MAX;
    let mut max_price = f32::MIN;

    for bar in price_bars {
        if bar.low < min_price {
            min_price = bar.low;
        }
        if bar.high > max_price {
            max_price = bar.high;
        }
    }

    (min_price, max_price)
}

fn get_price_position_y(price: f32, min_price: f32, price_span: f32, rect: Rect) -> f32 {
    let t = (price - min_price) / price_span; // 0..1
    rect.bottom() - t * rect.height()
}

fn get_price_color(is_bull: bool) -> Color32 {
    if is_bull {
        Color32::from_rgb(100, 200, 100)
    } else {
        Color32::from_rgb(200, 100, 100)
    }
}

fn handle_mouse_hover(
    chart: &mut ChartState,
    pointer_pos: Pos2,
    rect: Rect,
    price_bars: &[PriceBar],
) {
    if price_bars.is_empty() {
        chart.hover_price = None;
        chart.hover_index = None;
        return;
    }

    const BASE_CANDLE_WIDTH: f32 = 6.0;
    const BASE_GAP: f32 = 1.0;

    let candle_width = BASE_CANDLE_WIDTH * chart.zoom;
    let gap = BASE_GAP * chart.zoom;
    let total_candle = candle_width + gap;

    let x_offset = pointer_pos.x - rect.left() - chart.pan_offset;
    let index = (x_offset / total_candle).floor() as usize;

    if index < price_bars.len() {
        chart.hover_index = Some(index);

        let (min_price, max_price) = get_price_bounds(price_bars);
        let price_span = (max_price - min_price).max(1.0);

        let t = (rect.bottom() - pointer_pos.y) / rect.height();
        let price = min_price + t * price_span;

        chart.hover_price = Some(price);
    } else {
        chart.hover_index = None;
        chart.hover_price = None;
    }
}