use eframe::egui::{self, Ui};

pub enum Screens {
    Start,
    Settings,
    Game,
}

pub struct Gui {
    screen: Screens,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            screen: Screens::Start,
        }
    }

    pub fn start_screen(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Adventure game");
            if ui.button("Start").clicked() {
                self.screen = Screens::Game;
            }
        });
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screens::Start => self.start_screen(ui),
                _ => {}
            }
        });
    }
}
