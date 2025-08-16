mod maps;

use egui::{Color32, Stroke};
use maps::Map;

use crate::context::Context;

pub struct Radar {
    context: Context,
    map: Option<Map>,
}

impl Radar {
    pub fn new(context: Context) -> Self {
        Radar { context, map: None }
    }
}

impl eframe::App for Radar {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Perform reads to see current map, player positions and view angles.

        // TODO:
        // if self.map.as_ref() != Some(&new_map) {
        //  self.map = Some(new_map);
        // }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(map) = self.map {
                let response = ui.add(map.image());

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
