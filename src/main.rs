#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod context;
mod offsets;
mod radar;

use clap::Parser;
use context::Context;
use memflow::prelude::v1::*;

const OS: &str = if cfg!(target_os = "windows") {
    "win32"
} else {
    "linux"
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    connector: String,
}

fn main() -> eframe::Result {
    use crate::radar::Radar;

    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let args = Arguments::parse();

    let inventory = Inventory::scan();
    let os = inventory
        .builder()
        .connector(&args.connector)
        .os(OS)
        .build()
        .unwrap();

    let context = Context::new(os);

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Radar::new(context)))
        }),
    )
}
