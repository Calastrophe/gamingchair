use crate::Context;
use egui::Rect;

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

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(image) = self.context.map.image() {
                let scene = egui::Scene::new().zoom_range(0.5..=2.5);

                scene.show(ui, &mut self.scene_rect, |ui| {
                    let zeroing = self.context.map.zeroing();
                    let scale = self.context.map.scale();
                    let image_scaling = ui.add(image).rect.bottom() / 1024.0;
                    let painter = ui.painter();

                    self.context
                        .players()
                        .filter(|p| p.health > 0)
                        .for_each(|player| player.draw(ui, painter, zeroing, scale, image_scaling));
                });
            }
        });
    }
}
