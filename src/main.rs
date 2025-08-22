#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod context;
mod offsets;
mod radar;

use std::{ffi::OsString, thread, time::Duration};

use clap::Parser;
use context::Context;
use memflow::prelude::v1::*;
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW},
};

use std::sync::{Arc, LazyLock, Mutex};
//todo; find a clean location for this
static TOPMOST_WINDOW: LazyLock<Arc<Mutex<bool>>> = LazyLock::new(|| Arc::new(Mutex::new(false)));

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

    start_window_monitor();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_fullscreen(true)
            .with_always_on_top()
            .with_transparent(true),
        renderer: eframe::Renderer::Glow,
        multisampling: 1,
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

fn start_window_monitor() {
    thread::spawn(|| {
        loop {
            unsafe {
                let hwnd = GetForegroundWindow();
                if let Some(title) = get_window_title(hwnd) {
                    let is_cs2 = title == "Counter-Strike 2";
                    if let Ok(mut topmost_is_cs2) = TOPMOST_WINDOW.lock() {
                        if *topmost_is_cs2 != is_cs2 {
                            *topmost_is_cs2 = is_cs2;
                            if is_cs2 {
                                println!("Counter Strike 2 is now active");
                            } else {
                                println!("Counter Strike 2 is no longer active (now: {})", title);
                            }
                        }
                    } 
                }
            }
            // Sleep for 30ms
            thread::sleep(Duration::from_millis(30));
        }
    });
}




unsafe fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe {
        let title_length = GetWindowTextLengthW(hwnd);

        if title_length == 0 {
            return None;
        }

        let mut buffer: Vec<u16> = vec![0; (title_length + 1) as usize];
        let chars_copied = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

        if chars_copied == 0 {
            return None;
        }

        buffer.truncate(chars_copied as usize);
        String::from_utf16(&buffer).ok()
    }
}


