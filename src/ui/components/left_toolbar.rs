use eframe::egui::Ui;

pub fn draw_left_toolbar(ui: &mut Ui) {
    ui.vertical(|ui| {
        draw_account_balance(ui);
        draw_positions(ui);
        draw_order_form(ui);
        draw_indicators(ui);
    });
}

fn draw_account_balance(ui: &mut Ui) {
    ui.label("- ACCOUNT & BALANCE");
}

fn draw_positions(ui: &mut Ui) {
    ui.label("- POSITIONS");
}

fn draw_order_form(ui: &mut Ui) {
    ui.label("- ORDER FORM");
}

fn draw_indicators(ui: &mut Ui) {
    ui.label("- INDICATORS");
}