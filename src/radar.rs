use crate::Context;
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
                let response = ui.add(image);

                // TODO: Iterate over the players and draw their position and view angles.
                let painter = ui.painter();
                painter.circle(
                    response.rect.center(),
                    2.0,
                    Color32::RED,
                    Stroke::new(2.0, Color32::WHITE),
                );
            }
        });
    }
}
