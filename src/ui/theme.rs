use eframe::egui::{
    Color32, Context, CornerRadius, Margin, Shadow, Stroke, TextStyle, Vec2, Visuals,
};

pub fn default_theme(ctx: &Context) {
    let mut vis = Visuals::dark();

    vis.window_fill = Color32::from_rgba_unmultiplied(20, 20, 20, 200);
    vis.panel_fill = Color32::from_rgba_unmultiplied(35, 35, 35, 200);

    vis.window_corner_radius = CornerRadius::same(5);
    vis.menu_corner_radius = CornerRadius::same(5);

    vis.window_shadow = Shadow {
        offset: [0, 10],
        blur: 25,
        spread: 0,
        color: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
    };

    vis.widgets.noninteractive.bg_stroke =
        Stroke::NONE;

    vis.override_text_color = Some(Color32::from_rgba_unmultiplied(235, 235, 235, 200));

    ctx.set_visuals(vis);

    let mut style = (*ctx.style()).clone();

    style.spacing.window_margin = Margin::symmetric(16, 12);
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);

    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
    style.visuals.widgets.active.corner_radius = CornerRadius::same(4);

    style.visuals.text_edit_bg_color = Some(Color32::from_rgba_unmultiplied(20, 20, 20, 200));

    style.text_styles.get_mut(&TextStyle::Heading).map(|ts| {
        ts.size = 18.0;
    });
    style.text_styles.get_mut(&TextStyle::Body).map(|ts| {
        ts.size = 14.0;
    });
    style.text_styles.get_mut(&TextStyle::Button).map(|ts| {
        ts.size = 8.0;
    });
    style.text_styles.get_mut(&TextStyle::Small).map(|ts| {
        ts.size = 12.0;
    });

    ctx.set_style(style);
}
