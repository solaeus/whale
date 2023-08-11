use eframe::{
    egui::{CentralPanel, Context},
    run_native, NativeOptions,
};

use crate::{Macro, MacroInfo, Result, Value};

pub struct Gui;

impl Macro for Gui {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "gui",
            description: "Display a value in a window.",
            group: "gui",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.clone();
        let native_options = NativeOptions::default();

        run_native(
            "MyApp",
            native_options,
            Box::new(|cc| Box::new(ValueDisplay::new(argument))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

#[derive(Default)]
struct ValueDisplay(Value);

impl ValueDisplay {
    fn new(value: Value) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        ValueDisplay(value)
    }
}

impl eframe::App for ValueDisplay {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.vertical(|ui| {
                for row in self.0.as_table().unwrap().rows() {
                    ui.horizontal(|ui| {
                        for cell in row {
                            ui.label(cell.to_string());
                        }
                    });
                }
            });
        });
    }
}
