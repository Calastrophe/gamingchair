#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod context;
mod offsets;
mod radar;

use clap::Parser;
use context::Context;
use memflow::prelude::v1::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    connector: Option<String>,
}

fn main() -> eframe::Result {
    use crate::radar::Radar;

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    color_eyre::install().unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let args = Arguments::parse();

    let inventory = Inventory::scan();

    let os_instance = if let Some(connector) = args.connector {
        inventory
            .builder()
            .connector(&connector)
            .os("win32")
            .build()
    } else {
        inventory.builder().os("native").build()
    }
    .unwrap();

    let context = Context::new(os_instance);

    eframe::run_native(
        "radar",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Radar::new(context)))
        }),
    )
}
