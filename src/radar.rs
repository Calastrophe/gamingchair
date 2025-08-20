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
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.context.update();
        ctx.request_repaint();

        // Perform the read of any data that we need.
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = self.context.map.image() {
                let (zero_x, zero_y) = self.context.map.zeroing();
                let scale = self.context.map.scale();

                let response = ui.add(image);
                let painter = ui.painter();

                let img_scaling = response.rect.bottom() / 1024.0;

                for player in self.context.players() {
                    let pos_x = ((player.position.x - zero_x) / scale) * img_scaling;
                    let pos_y = ((player.position.y - zero_y) / -scale) * img_scaling;

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
                }
            }
        });
    }
}
