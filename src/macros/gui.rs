use eframe::{
    egui::{
        plot::{Line, Plot as EguiPlot, PlotPoints},
        CentralPanel, Context,
    },
    run_native, NativeOptions,
};

use crate::{Macro, MacroInfo, Result, Value};

pub struct Plot;

impl Macro for Plot {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "plot",
            description: "Render a list of numbers as a scatter plot graph.",
            group: "gui",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?.clone();

        run_native(
            "Whale Gui",
            NativeOptions::default(),
            Box::new(|_cc| Box::new(PlotGui::new(argument))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

struct PlotGui {
    data: Vec<Value>,
}

impl PlotGui {
    fn new(data: Vec<Value>) -> Self {
        Self { data }
    }
}

impl eframe::App for PlotGui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            EguiPlot::new("plot").show(ui, |plot_ui| {
                let points = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(index, value)| [index as f64, value.as_float().unwrap()])
                    .collect::<PlotPoints>();
                let line = Line::new(points);
                plot_ui.line(line);
            })
        });
    }
}
