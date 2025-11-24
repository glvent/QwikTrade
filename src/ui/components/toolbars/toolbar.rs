use eframe::egui::Ui;

use crate::app::App;
use crate::ui::components::toolbars::{top_toolbar, bottom_toolbar, left_toolbar, right_toolbar};

pub enum Toolbar {
    TopToolbar,
    BottomToolbar,
    LeftToolbar,
    RightToolbar,
}

impl Toolbar {
    pub fn draw(self, app: &mut App, ui: &mut Ui) {
        match self {
            Toolbar::TopToolbar => top_toolbar::draw(app, ui),
            Toolbar::BottomToolbar => bottom_toolbar::draw(app, ui),
            Toolbar::LeftToolbar => left_toolbar::draw(app, ui),
            Toolbar::RightToolbar => right_toolbar::draw(app, ui),
        }
    }
}