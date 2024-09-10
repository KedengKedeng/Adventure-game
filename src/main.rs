#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

pub mod world;
pub mod visuals;
pub mod utils;

//TODO: put in a .env file
const WITH_GUI: bool = true;

fn main() {
    if WITH_GUI {
        gui().unwrap();
    }
}

fn gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Adventure game",
        options,
        Box::new(|_cc| Ok(Box::new(visuals::screen::gui::Gui::new()))),
    )
}
