use eframe::egui;

use crate::app::App;

pub fn draw_central_panel(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("[TOOLS PLACEHOLDER]");

        ui.separator();

        let available_size = ui.available_size();
        let (rect, _res) = ui.allocate_exact_size(available_size, egui::Sense::hover());
        let painter = ui.painter_at(rect);

        painter.rect_filled(
            rect,
            4.0,
            egui::Color32::from_rgb(17, 17, 17),
        );

        painter.rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(0.5, egui::Color32::from_rgb(20, 20, 20)),
            egui::StrokeKind::Outside,
        );

        if app.market.price_bars().is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Loading data...",
                egui::TextStyle::Heading.resolve(ui.style()),
                egui::Color32::from_rgb(238, 238, 238)
            );
        }

        // Find min and max to scale Y axis...
        let mut min_price = f32::MAX;
        let mut max_price = f32::MIN;

        for bar in app.market.price_bars() {
            if bar.low < min_price { min_price = bar.low };
            if bar.high > max_price { max_price = bar.high };
        }

        let price_span = (max_price - min_price).max(1.0);

        // Maps price to a y coordinate within the chart rect...
        let price_to_y = |price: f32| -> f32 {
            let t = (price - min_price) / price_span; // 0..1
            rect.bottom() - t * rect.height()
        };

        // Candle sizing...
        let candle_width = 6.0_f32;
        let gap = 1.0_f32;
        let total_candle = candle_width + gap;
        let cap = (rect.width() / total_candle).floor() as usize;

        /* TODO: implement as a method after abstracting logic further...
        // Handles which bars to show...
        let len = app.market.price_bars().len();
        let start = if len > cap { len - cap } else { 0 };
        let bars = &app.market.price_bars()[start..];
        */ 

        let bars = app.market.price_bars();

        for (i, bar) in bars.iter().enumerate() {
            let x = rect.left() + (i as f32) * total_candle;
            let x_center = x + candle_width * 0.5;

            let y_open = price_to_y(bar.open);
            let y_close= price_to_y(bar.close);
            let y_high = price_to_y(bar.high);
            let y_low = price_to_y(bar.low);

            let is_bull = bar.close >= bar.open;

            let candle_color = if is_bull {
                egui::Color32::from_rgb(100, 200, 100)
            } else {
                egui::Color32::from_rgb(200, 100, 100)
            };

            // Candle wick...
            painter.line_segment(
                [
                    /* In the future when we introduce custom price bar coloring
                    we will need 2 seperate line segements for high and low... */
                    egui::Pos2::new(x_center, y_high),
                    egui::Pos2::new(x_center, y_low)
                ],
                egui::Stroke::new(1.0, candle_color),
            );

            // Candle body...
            let top = y_open.min(y_close);
            let bottom = y_open.max(y_close);
            let body_rect = egui::Rect::from_two_pos(
                egui::Pos2::new(x, top),
                egui::Pos2::new(x + candle_width, bottom),
            );

            painter.rect_filled(body_rect, 1.0, candle_color);
            painter.rect_stroke(
                body_rect, 
                1.0, 
                egui::Stroke::new(0.2, candle_color), 
                egui::StrokeKind::Outside
            );

            /* Optional: show min/max labels.
            painter.text(
                egui::Pos2::new(rect.left() + 4.0, rect.top() + 4.0),
                egui::Align2::LEFT_TOP,
                format!("{:.2}", max_price),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::from_rgb(200, 200, 220),
            );
            painter.text(
                egui::Pos2::new(rect.left() + 4.0, rect.bottom() - 4.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{:.2}", min_price),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::from_rgb(200, 200, 220),
            ); */
        }

        ctx.request_repaint();
    });
}

pub fn draw_top_panel(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("QwikTrade Prototype");

            ui.separator();

            ui.label("Symbol:");
            ui.text_edit_singleline(&mut app.symbol);

            ui.label("Timeframe:");
            ui.text_edit_singleline(&mut app.timeframe);

            ui.separator();

            if ui.button("Connect").clicked() {
                app.status_message = format!("Connecting to {} @ {}", app.symbol, app.timeframe);
            }

            if ui.button("Disconnect").clicked() {
                app.status_message = "Disconnected".to_owned();
            }

            ui.separator();

            ui.label(format!("Ticks: {}", app.tick_counter))
        })
    });
}

pub fn draw_left_panel(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading("Controls");

            ui.separator();

            ui.label("[CONTROLS PLACEHOLDER]");
            ui.label("- ACCOUNT & BALANCE");
            ui.label("- POSITIONS");
            ui.label("- ORDER FORM");
            ui.label("- INDICATORS");

            ui.separator();

            ui.label(format!("Status: {}", app.status_message));
        });
}
