use eframe::egui;

use crate::app::App;

pub fn draw_central_panel(ctx: &egui::Context) {
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

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "[CHART PLACEHOLDER]",
            egui::TextStyle::Heading.resolve(ui.style()),
            egui::Color32::from_rgb(238, 238, 238)
        );

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
            ui.label("[- ACCOUNT & BALANCE]");
            ui.label("[- POSITIONS]");
            ui.label("[- ORDER FORM]");
            ui.label("[- INDICATORS]");

            ui.separator();

            ui.label("Status:");
            ui.monospace(&app.status_message);
        });
}
