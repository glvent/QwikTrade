use eframe::egui::{
    Align2, Color32, Painter, Pos2, Rect, Sense, Stroke, StrokeKind, TextStyle, Ui,
};

use crate::app::App;
use crate::models::PriceBar;

pub fn draw_chart(app: &mut App, ui: &mut Ui) {
    let available_size = ui.available_size();
    let (rect, _res) = ui.allocate_exact_size(available_size, Sense::hover());
    let painter = ui.painter_at(rect);

    let price_bars = app.market.price_bars();

    draw_chart_background(&painter, rect);

    if price_bars.is_empty() {
        draw_loading_message(&painter, rect, ui);
        return;
    }

    draw_candles(rect, &painter, price_bars);
}

fn draw_chart_background(painter: &Painter, rect: Rect) {
    painter.rect_filled(rect, 4.0, Color32::from_rgb(17, 17, 17));

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
        Color32::from_rgb(238, 238, 238),
    );
}

fn draw_candles(rect: Rect, painter: &Painter, price_bars: &[PriceBar]) {
    let (min_price, max_price) = get_price_bounds(price_bars);
    let price_span = (max_price - min_price).max(1.0);

    // Candle sizing constants...
    const CANDLE_WIDTH: f32 = 6.0;
    const GAP: f32 = 1.0;
    const TOTAL_CANDLE: f32 = CANDLE_WIDTH + GAP;

    /* TODO: implement as a method after abstracting logic further...
    // Handles which bars to show...
    let cap = (rect.width() / TOTAL_CANDLE).floor() as usize;
    let len = price_bars.len();
    let start = if len > cap { len - cap } else { 0 };
    let bars = &price_bars[start..];
    */

    for (i, bar) in price_bars.iter().enumerate() {
        let x = rect.left() + (i as f32) * TOTAL_CANDLE;
        let x_center = x + CANDLE_WIDTH * 0.5;

        let y_open = get_price_position_y(bar.open, min_price, price_span, rect);
        let y_close = get_price_position_y(bar.close, min_price, price_span, rect);
        let y_high = get_price_position_y(bar.high, min_price, price_span, rect);
        let y_low = get_price_position_y(bar.low, min_price, price_span, rect);

        let candle_color = get_candle_color(bar.close >= bar.open);

        draw_candle(
            painter,
            x,
            x_center,
            y_open,
            y_close,
            y_high,
            y_low,
            CANDLE_WIDTH,
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

/* Optional draw price labels function:
fn draw_price_labels(painter: Painter, rect: Rect, ui: &Ui, min_price: f32, max_price: f32) {
    painter.text(
        Pos2::new(rect.left() + 4.0, rect.top() + 4.0),
        Align2::LEFT_TOP,
        format!("{:.2}", max_price),
        TextStyle::Body.resolve(ui.style()),
        Color32::from_rgb(200, 200, 220),
    );
    painter.text(
        Pos2::new(rect.left() + 4.0, rect.bottom() - 4.0),
        Align2::LEFT_BOTTOM,
        format!("{:.2}", min_price),
        TextStyle::Body.resolve(ui.style()),
        Color32::from_rgb(200, 200, 220),
    );
}
*/

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

fn get_candle_color(is_bull: bool) -> Color32 {
    if is_bull {
        Color32::from_rgb(100, 200, 100)
    } else {
        Color32::from_rgb(200, 100, 100)
    }
}