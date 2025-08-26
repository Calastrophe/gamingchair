use crate::util::generate_arc;
use crate::{Context, context::player::relation::Relation};
use egui::{Color32, Rect, Stroke};

pub struct Radar {
    context: Context,
    scene_rect: Rect,
}

impl Radar {
    pub fn new(context: Context) -> Self {
        Radar {
            context,
            scene_rect: Rect::ZERO,
        }
    }
}

impl eframe::App for Radar {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.context.update();
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = self.context.map.image() {
                let scene = egui::Scene::new().zoom_range(0.5..=2.5);

                scene.show(ui, &mut self.scene_rect, |ui| {
                    let (zero_x, zero_y) = self.context.map.zeroing();
                    let scale = self.context.map.scale();
                    let img_scaling = ui.add(image).rect.bottom() / 1024.0;
                    let painter = ui.painter();

                    self.context
                        .players()
                        .filter(|p| p.health > 0)
                        .for_each(|player| {
                            let player_position: egui::Pos2 = (
                                ((player.position.x - zero_x) / scale) * img_scaling,
                                ((player.position.y - zero_y) / -scale) * img_scaling,
                            )
                                .into();

                            let color = match player.relation {
                                Relation::Unknown | Relation::Spectator => Color32::GRAY,
                                Relation::Enemy => Color32::RED,
                                Relation::Teammate => Color32::BLUE,
                                Relation::Local => Color32::GREEN,
                            };

                            let yaw_radians = player.yaw.to_radians();
                            let line_length = 14.0 * img_scaling;
                            let direction_vec = egui::Vec2::new(
                                line_length * yaw_radians.cos(),
                                -line_length * yaw_radians.sin(),
                            );

                            painter.arrow(
                                player_position,
                                direction_vec,
                                Stroke::new(2.0, Color32::WHITE),
                            );

                            let circ_radius = 8.0 * img_scaling;

                            painter.circle(player_position, circ_radius, color, Stroke::NONE);

                            let health_percentage = player.health as f32 / 100.0;
                            let start_angle = -std::f32::consts::FRAC_PI_2;
                            let end_angle =
                                start_angle + 2.0 * std::f32::consts::PI * health_percentage;

                            generate_arc(
                                painter,
                                player_position,
                                circ_radius,
                                start_angle,
                                end_angle,
                                Stroke::new(1.5, Color32::WHITE),
                            );

                            painter.text(
                                player_position + egui::Vec2::new(0.0, 20.0),
                                egui::Align2::CENTER_CENTER,
                                &player.name,
                                egui::FontId::monospace(8.0),
                                Color32::WHITE,
                            );

                            painter.text(
                                player_position + egui::Vec2::new(0.0, 30.0),
                                egui::Align2::CENTER_CENTER,
                                player.current_weapon.as_str(),
                                egui::FontId::monospace(8.0),
                                Color32::WHITE,
                            );
                        });
                });
            }
        });
    }
}
