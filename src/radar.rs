use crate::Context;
use egui::{Align2, Color32, CornerRadius, FontId, Layout, Pos2, ProgressBar, Rect, Stroke, Vec2};

const ACTUAL_IMAGE_SIZE: f32 = 1024.0;
const SIGHT_LINE_LENGTH: f32 = 14.0;
const SCOPED_LINE_LENGTH: f32 = 1000.0;
const CIRCLE_RADIUS: f32 = 8.0;
const MAX_HEALTH: f32 = 100.0;
const FONT_SIZE: f32 = 9.0;
const NAME_OFFSET: Vec2 = Vec2::new(0.0, 20.0);
const BOMB_CARRIER_OFFSET: Vec2 = Vec2::new(0.0, 30.0);
const CAUTION_OFFSET: Vec2 = Vec2::new(0.0, -25.0);
const UTILITY_OFFSET: Vec2 = Vec2::new(0.0, -22.0);
const UTILITY_SIZE: Vec2 = Vec2::new(12.0, 20.0);

mod util;

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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.context.update();
        ctx.request_repaint();

        if !self.context.information.in_game() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.add_space(ui.max_rect().height() / 2.0);
                    ui.spinner();
                    ui.label("Awaiting a match...");
                });
            });
            return;
        }

        // Draw the side panel responsible for showing general economy and loadouts.
        egui::SidePanel::right("economy_loadout").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let (friendlies, enemies) = self.context.entities.sides();

                friendlies
                    .iter()
                    .chain(enemies.iter())
                    .filter(|player| player.health > 0)
                    .for_each(|player| {
                        ui.label(&player.name);

                        ui.add_space(5.0);

                        // Draw their health, armor, and money all on a single line.
                        ui.horizontal(|ui| {
                            ui.image(egui::include_image!("../assets/icons/health.svg"));
                            ui.label(player.health.to_string());
                            ui.add_space(5.0);
                            ui.image(egui::include_image!("../assets/icons/armor.svg"));
                            ui.label(player.armor.to_string());

                            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                                ui.colored_label(Color32::GREEN, format!("${}", player.money));
                            });
                        });

                        ui.add_space(5.0);

                        // Grab the loadout and partition weapons and utility.
                        let (weapons, utilities) = player.partitioned_loadout();

                        // Draw separate horizontal components for weapons and utility images.
                        ui.horizontal(|ui| {
                            weapons
                                .iter()
                                .filter_map(|weapon| weapon.image())
                                .for_each(|image| {
                                    ui.image(image.source(ctx));
                                });
                        });

                        ui.horizontal(|ui| {
                            utilities
                                .iter()
                                .filter_map(|utility| utility.image())
                                .for_each(|image| {
                                    ui.image(image.source(ctx));
                                    ui.add_space(5.0);
                                });
                        });

                        ui.add_space(5.0);
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let current_map = &self.context.information.current_map;
            let bomb = &self.context.information.bomb;

            if bomb.is_planted {
                // Determine which fuse we need to draw based on if we are defusing or not.
                let (timestamp, length, color) = if bomb.is_defusing {
                    (
                        bomb.defuse_stamp.unwrap(),
                        bomb.defuse_length,
                        Color32::BLUE,
                    )
                } else {
                    (bomb.fuse_stamp.unwrap(), bomb.fuse_length, Color32::ORANGE)
                };

                let elapsed = timestamp.elapsed().as_secs_f32();
                let remaining = (length - elapsed).max(0.0);
                let percentage = (remaining / length).clamp(0.0, 1.0);

                // We use the response to determine where to draw the text.
                let resp = ui.add(
                    ProgressBar::new(percentage)
                        .corner_radius(CornerRadius::same(3))
                        .fill(color),
                );

                // Draw a centered text of the remaining seconds on the fuse.
                ui.painter().text(
                    resp.rect.center(),
                    Align2::CENTER_CENTER,
                    format!("{:.1}s", remaining),
                    FontId::monospace(FONT_SIZE),
                    Color32::WHITE,
                );
            }

            if let Some(image) = current_map.image() {
                let scene = egui::Scene::new().zoom_range(0.5..=2.5);

                scene.show(ui, &mut self.scene_rect, |ui| {
                    let (offset_x, offset_y) = current_map.zeroing();
                    let scale = current_map.scale();
                    let image_scaling = ui.add(image).rect.bottom() / ACTUAL_IMAGE_SIZE;
                    let painter = ui.painter();

                    self.context
                        .entities
                        .players()
                        .filter(|p| p.health > 0)
                        .for_each(|player| {
                            // Convert the player's coordinates into map coordinates then scaled by the current image scaling.
                            let position = Pos2::new(
                                ((player.position.x - offset_x) / scale) * image_scaling,
                                ((player.position.y - offset_y) / -scale) * image_scaling,
                            );

                            // Create a vector in the direction they are looking.
                            let yaw = player.yaw.to_radians();
                            let line_length = if player.is_scoped {
                                SCOPED_LINE_LENGTH
                            } else {
                                SIGHT_LINE_LENGTH
                            };
                            let direction_vector =
                                Vec2::new(line_length * yaw.cos(), -line_length * yaw.sin());

                            // Draw a straight line if they are scoped, otherwise an arrow.
                            if player.is_scoped {
                                let end = position + direction_vector;
                                painter.line_segment(
                                    [position, end],
                                    Stroke::new(1.0, Color32::WHITE),
                                );
                            } else {
                                painter.arrow(
                                    position,
                                    direction_vector,
                                    Stroke::new(2.0, Color32::WHITE),
                                );
                            }

                            // Draw a circle to indicate player's position.
                            let radius = CIRCLE_RADIUS * image_scaling;
                            painter.circle(position, radius, player.color(), Stroke::NONE);

                            let health_percentage = player.health as f32 / MAX_HEALTH;
                            let start_angle = -std::f32::consts::FRAC_PI_2;
                            let end_angle =
                                start_angle + 2.0 * std::f32::consts::PI * health_percentage;

                            // Draw a ring around the player's circle indicating remaining health.
                            util::generate_arc(
                                painter,
                                position,
                                radius,
                                start_angle,
                                end_angle,
                                Stroke::new(1.5, Color32::WHITE),
                            );

                            // Display player's name a little below their circle.
                            painter.text(
                                position + NAME_OFFSET,
                                Align2::CENTER_CENTER,
                                &player.name,
                                FontId::monospace(FONT_SIZE),
                                Color32::WHITE,
                            );

                            // Display an indicator if the player is the bomb carrier.
                            if player.is_bomb_carrier {
                                painter.text(
                                    position + BOMB_CARRIER_OFFSET,
                                    Align2::CENTER_CENTER,
                                    "[C4]",
                                    FontId::monospace(FONT_SIZE),
                                    Color32::ORANGE,
                                );
                            }

                            // Caution opposing enemy players with snipers!
                            if player.is_enemy() && player.current_weapon.is_sniper() {
                                painter.text(
                                    position + CAUTION_OFFSET,
                                    Align2::CENTER_CENTER,
                                    "!",
                                    FontId::monospace(FONT_SIZE * 2.0),
                                    Color32::ORANGE,
                                );
                            }

                            // Display any utility being held by all players.
                            if player.current_weapon.is_utility() {
                                let image = player.current_weapon.image().unwrap();
                                let rect =
                                    Rect::from_center_size(position + UTILITY_OFFSET, UTILITY_SIZE);

                                image.paint_at(ui, rect);
                            }
                        });
                });
            }
        });
    }
}
