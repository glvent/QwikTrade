use eframe::egui;

use crate::app::TradeApp;

pub fn draw_central_panel(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("[CHART PLACEHOLDER]");

        ui.label("[CANDLESTICK PLACEHOLDER");

        ui.separator();

        //let available_size = ui.available_size();
        //let (rect, response) = ui.allocate_exact_size(available_size, egui::Sense::hover());
        //let painter = ui.painter();

        ctx.request_repaint();
    });
}

pub fn draw_top_panel(app: &mut TradeApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("TradeEZ Prototype");

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

