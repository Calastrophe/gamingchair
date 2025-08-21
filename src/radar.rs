use crate::{Context, context::player::relation::Relation};
use egui::{Color32, Stroke};

pub struct Radar {
    context: Context,
}

impl Radar {
    pub fn new(context: Context) -> Self {
        Radar { context }
    }
}

impl eframe::App for Radar {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.context.update();
        ctx.request_repaint();
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));

        egui::Window::new("Gaming chair").show(ctx, |ui| {
            if let Some(image) = self.context.map.image() {
                let (zero_x, zero_y) = self.context.map.zeroing();
                let scale = self.context.map.scale();
                let img_scaling = ui.add(image).rect.bottom() / 1024.0;
                let painter = ui.painter();

                self.context
                    .players()
                    .filter(|p| p.health > 0)
                    .for_each(|player| {
                        let (pos_x, pos_y) = (
                            ((player.position.x - zero_x) / scale) * img_scaling,
                            ((player.position.y - zero_y) / -scale) * img_scaling,
                        );

                        let color = match player.relation {
                            Relation::Unknown | Relation::Spectator => Color32::GRAY,
                            Relation::Enemy => Color32::RED,
                            Relation::Teammate => Color32::BLUE,
                            Relation::Local => Color32::GREEN,
                        };

                        painter.circle(
                            (pos_x, pos_y).into(),
                            4.0 * img_scaling,
                            color,
                            Stroke::new(1.0, Color32::WHITE),
                        );

                        let yaw_radians = player.yaw.to_radians();
                        let line_length = 8.0 * img_scaling;

                        let end_x = pos_x + line_length * yaw_radians.cos();
                        let end_y = pos_y - line_length * yaw_radians.sin();

                        painter.line_segment(
                            [(pos_x, pos_y).into(), (end_x, end_y).into()],
                            Stroke::new(3.0, Color32::WHITE),
                        );
                    });
            }
        });
    }
}
