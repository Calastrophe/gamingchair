use crate::Context;
use egui::Rect;

const ACTUAL_IMAGE_SIZE: f32 = 1024.0;

mod overlay;

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

        // If enabled, draw the in-game overlay.

        // Draw the side panel responsible for showing general economy and loadouts.
        // egui::SidePanel::right("economy_loadout").show(ctx, |ui| {
        //     if !self.context.information.in_game() {
        //         return;
        //     }

        //     let (friendlies, enemies) = self.context.players.sides();

        //     ui.vertical(|ui| {
        //         ui.label("friendlies");
        //         for friendly in friendlies {
        //             ui.label(&friendly.name);
        //             ui.horizontal(|ui| {
        //                 ui.label(friendly.health.to_string());

        //                 for weapon in &friendly.loadout {
        //                     if let Some(image) = weapon.image() {
        //                         ui.image(image.source(ctx));
        //                     }
        //                 }
        //             });
        //         }

        //         ui.separator();
        //         ui.label("enemies");
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            let current_map = self.context.information.current_map;

            if let Some(image) = current_map.image() {
                let scene = egui::Scene::new().zoom_range(0.5..=2.5);

                scene.show(ui, &mut self.scene_rect, |ui| {
                    let zeroing = current_map.zeroing();
                    let scale = current_map.scale();
                    let image_scaling = ui.add(image).rect.bottom() / ACTUAL_IMAGE_SIZE;
                    let painter = ui.painter();

                    self.context
                        .players
                        .iter()
                        .filter(|p| p.health > 0)
                        .for_each(|player| player.draw(ui, painter, zeroing, scale, image_scaling));
                });
            }
        });
    }
}
