use eframe::{
    egui::{CentralPanel, Context, Layout, Widget, Window},
    emath::Align,
    epaint::Color32,
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
            Value::String(string) => ui.label(string),
            Value::Float(float) => ui.label(float.to_string()),
            Value::Integer(int) => ui.label(int.to_string()),
            Value::Boolean(bool) => ui.label(bool.to_string()),
            Value::List(list) => ui.add(&Value::Table(Table::from(list))),
            Value::Map(map) => ui.add(&Value::Table(Table::from(map))),
            Value::Table(table) => {
                let collapsing_table = ui.collapsing("table", |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .auto_shrink([true, false])
                        .cell_layout(Layout::left_to_right(Align::LEFT))
                        .columns(Column::remainder(), table.column_names().len())
                        .min_scrolled_height(25.0)
                        .header(25.0, |mut header| {
                            for column_name in table.column_names() {
                                header.col(|ui| {
                                    ui.heading(column_name);
                                });
                            }
                        })
                        .body(|mut body| {
                            for row_data in table.rows() {
                                body.row(20.0, |mut row| {
                                    for column_data in row_data {
                                        row.col(|ui| {
                                            ui.add(column_data);
                                        });
                                    }
                                })
                            }
                        });
                });

                collapsing_table
                    .body_response
                    .unwrap_or(collapsing_table.header_response)
            }
            Value::Function(_) => todo!(),
            Value::Empty => todo!(),
        }
    }
}

struct ValueDisplay {
    data: Value,
    show_data_types: bool,
}

impl ValueDisplay {
    fn new(data: Value) -> Self {
        Self {
            data,
            show_data_types: false,
        }
    }
}

impl eframe::App for ValueDisplay {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("whale").show(ctx, |ui| ui.add(&self.data));
    }
}
