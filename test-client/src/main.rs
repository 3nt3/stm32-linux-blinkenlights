#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use blinkenlights_protocol::{Command, Level};
use eframe::egui;
use log::error;

mod usb;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    level: Level,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // TODO
            level: Level::Low,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("STM32 Blinkenlights");
            if ui.button("Toggle LEDs").clicked() {
                let cmd = Command::SetAll(if self.level.into() {
                    Level::Low
                } else {
                    Level::High
                });

                usb::handle_command(cmd).unwrap_or_else(|err| {
                    error!("Error handling command: {}", err);
                });

                self.level = if self.level.into() {
                    Level::Low
                } else {
                    Level::High
                };
            }
        });
    }
}
