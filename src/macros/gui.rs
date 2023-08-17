use eframe::{
    egui::{
        plot::{Bar, BarChart, Line, Plot as EguiPlot, PlotPoints},
        CentralPanel, Context,
    },
    epaint::Color32,
    run_native, NativeOptions,
};

use crate::{Error, Macro, MacroInfo, Result, Value};

pub struct BarGraph;

impl Macro for BarGraph {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "bar_graph",
            description: "Render a list of values as a bar graph.",
            group: "gui",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let mut data = Vec::new();

        for value in argument {
            let list = value.as_fixed_len_list(2)?;
            list[0].as_string()?;
            list[1].as_number()?;

            data.push(value.clone());
        }

        run_native(
            "bar_graph",
            NativeOptions::default(),
            Box::new(|_cc| Box::new(BarGraphGui::new(data))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

struct BarGraphGui {
    data: Vec<Value>,
}

impl BarGraphGui {
    fn new(data: Vec<Value>) -> Self {
        Self { data }
    }
}

impl eframe::App for BarGraphGui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut bars = Vec::with_capacity(self.data.len());
            let data = &self.data;

            for (index, value) in data.into_iter().enumerate() {
                let list = if let Ok(list) = value.as_list() {
                    list
                } else {
                    continue;
                };
                let name = if let Ok(name) = list[0].as_string() {
                    name
                } else {
                    continue;
                };
                let height = if let Ok(height) = list[1].as_float() {
                    height
                } else if let Ok(height) = list[1].as_int() {
                    height as f64
                } else {
                    continue;
                };
                let bar = Bar::new(index as f64, height).name(name);

                bars.push(bar);
            }

            EguiPlot::new("bar_graph").show(ui, |plot_ui| {
                plot_ui.bar_chart(BarChart::new(bars).color(Color32::RED));
            });
        });
    }
}

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
        let argument = argument.as_list()?;
        let mut floats = Vec::new();

        for value in argument {
            if let Ok(float) = value.as_float() {
                floats.push(float);
            } else if let Ok(integer) = value.as_int() {
                floats.push(integer as f64);
            } else {
                return Err(Error::expected_number(value.clone()));
            }
        }

        run_native(
            "plot",
            NativeOptions::default(),
            Box::new(|_cc| Box::new(PlotGui::new(floats))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

struct PlotGui {
    data: Vec<f64>,
}

impl PlotGui {
    fn new(data: Vec<f64>) -> Self {
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
                    .map(|(index, value)| [index as f64, *value])
                    .collect::<PlotPoints>();
                let line = Line::new(points);
                plot_ui.line(line);
            })
        });
    }
}
