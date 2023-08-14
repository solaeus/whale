use eframe::{
    egui::{CentralPanel, Context, Layout, Sense, Widget},
    emath::Align,
    epaint::{vec2, Color32},
    run_native, NativeOptions,
};
use egui_extras::{Column, TableBuilder};

use crate::{Macro, MacroInfo, Result, Table, Value};

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

        run_native(
            "Whale Gui",
            NativeOptions::default(),
            Box::new(|_cc| Box::new(ValueDisplay::new(argument))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

impl Widget for &Value {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        match self {
            Value::String(string) => ui.colored_label(Color32::GREEN, string.clone()),
            Value::Float(float) => ui.colored_label(Color32::RED, float.to_string()),
            Value::Integer(int) => ui.colored_label(Color32::BLUE, int.to_string()),
            Value::Boolean(bool) => ui.colored_label(Color32::GOLD, bool.to_string()),
            Value::List(list) => ui.add(&Value::Table(Table::from(list))),
            Value::Map(map) => ui.add(&Value::Table(Table::from(map))),
            Value::Table(table) => {
                let mut response = ui.allocate_response(vec2(0.0, 0.0), Sense::click());

                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .columns(Column::auto(), table.column_names().len())
                    .min_scrolled_height(25.0)
                    .header(25.0, |mut header| {
                        for column_name in table.column_names() {
                            header.col(|ui| {
                                response = ui.heading(column_name);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(20.0, table.rows().len(), |row_index, mut row| {
                            for row_data in table.rows() {
                                row.col(|ui| {
                                    for column_data in row_data {
                                        ui.add(column_data);
                                    }
                                });
                            }
                        });
                    });

                response
            }
            Value::Function(_) => todo!(),
            Value::Empty => todo!(),
        }
    }
}

struct ValueDisplay {
    data: Value,
}

impl ValueDisplay {
    fn new(data: Value) -> Self {
        Self { data }
    }
}

impl eframe::App for ValueDisplay {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.add(&self.data));
    }
}
