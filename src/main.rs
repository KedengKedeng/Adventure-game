#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

pub mod world;

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

    eframe::run_simple_native("Adventure game", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Adventure game");
                if ui.button("Start").clicked() {
                    todo!();
                }
            })
        });
    })
}
