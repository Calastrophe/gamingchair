mod equipment;
mod maps;
mod player;

use egui::{Color32, Stroke};
use maps::Map;

use crate::{context::Context, radar::equipment::Equipment};

pub struct Radar {
    context: Context,
    map: Option<Map>,
}

impl Radar {
    pub fn new(context: Context) -> Self {
        Radar {
            context,
            map: Some(Map::Inferno),
        }
    }
}

impl eframe::App for Radar {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Perform the read of any data that we need.
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

        egui::SidePanel::right("my_left_panel").show(ctx, |ui| {
            ui.collapsing("Ally", |ui| {
                ui.horizontal(|ui| {
                    ui.monospace("sdiybt");
                    ui.separator();
                    ui.add(Equipment::Helmet.image());
                    ui.add(Equipment::AK47.image());
                    ui.add(Equipment::Tec9.image());
                    ui.add(Equipment::Zeus.image());
                    ui.add(Equipment::Flashbang.image());
                    ui.add(Equipment::HE.image());
                    ui.add(Equipment::Smoke.image());
                    ui.separator();
                });
            });
            ui.collapsing("Enemy", |ui| {
                ui.horizontal(|ui| {
                    ui.monospace("sdiybt");
                    ui.separator();
                    ui.add(Equipment::Helmet.image());
                    ui.add(Equipment::AK47.image());
                    ui.add(Equipment::Tec9.image());
                    ui.add(Equipment::Zeus.image());
                    ui.add(Equipment::Flashbang.image());
                    ui.add(Equipment::HE.image());
                    ui.add(Equipment::Smoke.image());
                    ui.separator();
                });
            });
        });
    }
}
