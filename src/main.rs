use eframe::egui;

struct TradeApp {
    symbol: String,
    timeframe: String,
    status_message: String,
    tick_counter: u64,
}

impl Default for TradeApp {
    fn default() -> Self {
        // Placeholder data...
        TradeApp {
            symbol: "BTC-USDT".to_owned(),
            timeframe: "1m".to_owned(),
            status_message: "Disconnected".to_owned(),
            tick_counter: 0,
        }
    }
}

impl eframe::App for TradeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick_counter += 1;

        // TODO: abstract component logic
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TradeEZ Prototype");

                ui.separator();

                ui.label("Symbol:");
                ui.text_edit_singleline(&mut self.symbol);

                ui.label("Timeframe:");
                ui.text_edit_singleline(&mut self.timeframe);

                ui.separator();

                if ui.button("Connect").clicked() {
                    self.status_message = format!("Connecting to {} @ {}", self.symbol, self.timeframe);
                }

                if ui.button("Disconnect").clicked() {
                    self.status_message = "Disconnected".to_owned();
                }

                ui.separator();

                ui.label(format!("Ticks: {}", self.tick_counter))
            })

            
        });

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
}

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_title("TradeEZ");

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "TradeEZ", 
        native_options,
        Box::new(|_cc| Ok(Box::new(TradeApp::default()))),
    )
}
